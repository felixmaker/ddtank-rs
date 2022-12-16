# ddtank-rs
DDTANK-RS is an easy-to-use ddtank login tool, it supports Linux, Windows and MacOS.
It's recommanded to play ddtank in web browsers. However you can play it in an standalone 
flashplayer maybe available from 
[web archive](https://archive.org/details/standaloneflashplayers). For Windows user,
downlaod it from [flash.cn](https://www.flash.cn/support/debug-downloads)

## Requirements
 - Sciter-JS-SDK 5.0.1.4
 - Rustc 2021
 - Standalone flash player 32

### build ui.rc
```
path-to-sciter-js-sdk/bin/windows/packfolder.exe "./src/ui" "./src/ui.rc" -binary
```

## Current Supported Platform
 - 7K7K

## TODO List
 - [ ] 4399
 - [ ] 7road
 - [ ] 7road Classic
 - [ ] Common login based on webview2
