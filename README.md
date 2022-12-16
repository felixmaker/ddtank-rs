# DDTANK-rs
DDTANK-rs is an easy-to-use ddtank login tool. It supports Linux, Windows and MacOS.
It's recommanded to play ddtank in web browsers. However you can play it in an standalone 
flashplayer(maybe available at [web archive](https://archive.org/details/standaloneflashplayers)). 
Windows user can downlaod a standalone flashplayer from [flash.cn](https://www.flash.cn/support/debug-downloads)

## Requirements
 - scite-js-sdk 5.0.1.4
 - Rustc 2021
 - Standalone flash player 32

### build ui.rc
```
path-to-sciter-js-sdk/bin/windows/packfolder.exe "./src/ui" "./src/ui.rc" -binary
```

## Current Supported Platform
 - [x] 7K7K
 - [ ] 4399
 - [ ] 7road
 - [ ] 7road Classic
 - [ ] Common login based on webview2
