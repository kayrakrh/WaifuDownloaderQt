# WaifuDownloaderQt
 A QT application that downloads images of waifus/nekos based on https://waifu.im and https://nekos.moe 
 
| Nekos.moe | Waifu.im |
| --- | --- |
| <img width="1136" height="1125" alt="image" src="https://github.com/user-attachments/assets/20fbae7e-340d-4a1f-bdca-c9d31d95be25" /> | <img width="1134" height="1123" alt="image" src="https://github.com/user-attachments/assets/af5c726d-9c30-4013-927a-f55e06da2604" /> |


### Requirements

* qt6-base-dev 
* libcurl4-openssl-dev 
* cmake

### Build
```bash
git clone https://github.com/kayrakrh/WaifuDownloaderQt
cd WaifuDownloaderQt
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
sudo make install
```

## About WaifuDownloaderQt
based on 
* [`CatgirlDownloaderQT`](https://github.com/KairaBegudiri/catgirldownloaderqt)
* [`NyarchLinux/CatgirlDownloader`](https://github.com/NyarchLinux/CatgirlDownloader)
* [`NyarchLinux/WaifuDownloader`](https://github.com/NyarchLinux/WaifuDownloader)

## Star History

<a href="https://github.com/KairaBegudiri/WaifuDownloaderQt/stargazers">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=kayrakrh/WaifuDownloaderQt&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=kayrakrh/WaifuDownloaderQt&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=kayrakrh/WaifuDownloaderQt&type=Date" />
 </picture>
</a>
