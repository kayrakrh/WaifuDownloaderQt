mod api;
mod config;
mod storage;

use api::{Rating, Source};
use config::SaveMode;
use slint::{ModelRc, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel};
use std::sync::{Arc, Mutex};

slint::include_modules!();

type Decoded = (Vec<u8>, SharedPixelBuffer<Rgba8Pixel>);

fn fetch_and_decode(source: Source, rating: Rating) -> anyhow::Result<Decoded> {
    let bytes = api::fetch_random(source, rating)?;
    let rgba = image::load_from_memory(&bytes)?.to_rgba8();
    let buf = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba.as_raw(),
        rgba.width(),
        rgba.height(),
    );
    Ok((bytes, buf))
}

fn labels<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> ModelRc<SharedString> {
    let v: Vec<SharedString> = items.iter().map(|i| f(*i).into()).collect();
    ModelRc::new(VecModel::from(v))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let current: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));

    ui.set_sources(labels(&Source::ALL, Source::name));
    ui.set_ratings(labels(&Rating::ALL, Rating::name));
    ui.set_save_modes(labels(&SaveMode::ALL, SaveMode::label));
    ui.set_placeholder("Click Refresh to load.".into());

    // Kayıtlı tercihi yükle
    ui.set_save_mode_index(config::load().save_mode.index());

    // Tercih değişince hemen diske yaz
    {
        let weak = ui.as_weak();
        ui.on_save_mode_changed(move |idx| {
            let Some(ui) = weak.upgrade() else { return };
            let cfg = config::Config { save_mode: SaveMode::from_index(idx) };
            if let Err(e) = config::save(&cfg) {
                ui.set_status(format!("Could not save settings: {e:#}").into());
            }
        });
    }

    // Refresh
    {
        let weak = ui.as_weak();
        let current = current.clone();
        ui.on_refresh(move || {
            let Some(ui) = weak.upgrade() else { return };
            let source = Source::ALL[ui.get_source_index() as usize];
            let rating = Rating::ALL[ui.get_rating_index() as usize];

            ui.set_loading(true);
            ui.set_status("Loading...".into());

            let weak = ui.as_weak();
            let current = current.clone();
            std::thread::spawn(move || {
                let result = fetch_and_decode(source, rating);
                let _ = slint::invoke_from_event_loop(move || {
                    let Some(ui) = weak.upgrade() else { return };
                    ui.set_loading(false);
                    match result {
                        Ok((bytes, buf)) => {
                            *current.lock().unwrap() = Some(bytes);
                            ui.set_picture(slint::Image::from_rgba8(buf));
                            ui.set_has_picture(true);
                            ui.set_status("Loaded.".into());
                        }
                        Err(e) => {
                            let msg = format!("{e:#}");
                            ui.set_status(format!("Error: {msg}").into());
                            rfd::MessageDialog::new()
                                .set_level(rfd::MessageLevel::Error)
                                .set_title("Error")
                                .set_description(msg)
                                .show();
                        }
                    }
                });
            });
        });
    }

    // Save
    {
        let weak = ui.as_weak();
        let current = current.clone();
        ui.on_save(move || {
            let Some(ui) = weak.upgrade() else { return };
            let Some(bytes) = current.lock().unwrap().clone() else { return };

            let result = match SaveMode::from_index(ui.get_save_mode_index()) {
                SaveMode::DefaultFolder => Some(storage::save_default(&bytes)),
                SaveMode::Ask => storage::save_ask(&bytes),
            };

            match result {
                None => {} // diyalog iptal edildi
                Some(Ok(path)) => {
                    ui.set_status(format!("Saved to '{}'.", path.display()).into());
                }
                Some(Err(e)) => {
                    rfd::MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning)
                        .set_title("Error")
                        .set_description(format!("Could not save: {e:#}"))
                        .show();
                }
            }
        });
    }

    ui.run()
}
