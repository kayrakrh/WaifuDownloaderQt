use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{collections::hash_map::RandomState, hash::BuildHasher, io::Read, time::Duration};

const USER_AGENT: &str = "WaifuDownloader/0.1";
const MAX_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    NekosMoe,
    WaifuIm,
}

impl Source {
    pub const ALL: [Source; 2] = [Source::NekosMoe, Source::WaifuIm];

    pub fn name(self) -> &'static str {
        match self {
            Source::NekosMoe => "Nekos.moe",
            Source::WaifuIm => "Waifu.im",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rating {
    Safe,
    Nsfw,
    Both,
}

impl Rating {
    pub const ALL: [Rating; 3] = [Rating::Safe, Rating::Nsfw, Rating::Both];

    pub fn name(self) -> &'static str {
        match self {
            Rating::Safe => "Safe",
            Rating::Nsfw => "NSFW",
            Rating::Both => "Both",
        }
    }

    /// Both: her istekte rastgele safe ya da nsfw seçer.
    fn resolve_nsfw(self) -> bool {
        match self {
            Rating::Safe => false,
            Rating::Nsfw => true,
            Rating::Both => RandomState::new().hash_one(0u8) & 1 == 1,
        }
    }
}

#[derive(Deserialize)]
struct NekosResponse {
    images: Vec<NekosImage>,
}
#[derive(Deserialize)]
struct NekosImage {
    id: String,
}

#[derive(Deserialize)]
struct WaifuResponse {
    items: Vec<WaifuImage>,
}
#[derive(Deserialize)]
struct WaifuImage {
    url: String,
}

fn http_get(url: &str) -> Result<Vec<u8>> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(15))
        .user_agent(USER_AGENT)
        .build();
    let resp = agent.get(url).call().with_context(|| format!("GET {url}"))?;
    let mut buf = Vec::new();
    resp.into_reader().take(MAX_BYTES).read_to_end(&mut buf)?;
    Ok(buf)
}

fn random_image_url(source: Source, nsfw: bool) -> Result<String> {
    let url = match source {
        Source::NekosMoe => {
            let raw = http_get(&format!("https://nekos.moe/api/v1/random/image?nsfw={nsfw}"))?;
            let r: NekosResponse = serde_json::from_slice(&raw).context("Nekos.moe JSON")?;
            let img = r.images.into_iter().next().context("Nekos.moe: boş sonuç")?;
            format!("https://nekos.moe/image/{}", img.id)
        }
        Source::WaifuIm => {
            let flag = if nsfw { "True" } else { "False" };
            let raw = http_get(&format!("https://api.waifu.im/images?IsNsfw={flag}"))?;
            let r: WaifuResponse = serde_json::from_slice(&raw).context("Waifu.im JSON")?;
            r.items.into_iter().next().context("Waifu.im: boş sonuç")?.url
        }
    };
    if !url.starts_with("http") {
        bail!("Invalid image URL");
    }
    Ok(url)
}

pub fn fetch_random(source: Source, rating: Rating) -> Result<Vec<u8>> {
    let url = random_image_url(source, rating.resolve_nsfw())?;
    http_get(&url)
}
