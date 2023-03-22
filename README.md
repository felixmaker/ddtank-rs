# DDTANK-rs
DDTANK-rs is an easy-to-use ddtank login tool. It supports Linux, Windows and MacOS.
It's recommanded to play ddtank in web browsers. However you can play it in an standalone 
flashplayer (maybe available at [web archive](https://archive.org/details/standaloneflashplayers)). 
Windows user can downlaod a standalone flashplayer from [flash.cn](https://www.flash.cn/support/debug-downloads)

## Lua Script API
Use these Lua script API to login game:
 - _G.userdata (username, password, server_id)
 - _G.agent (get, post, get_with)
 - _G.crypto (md5)

See [7k7k.lua](./scripts/7k7k.lua) for example.

## Requirements
 - scite-js-sdk 5.0.2.7 (sciter.dll)
 - Standalone flash player 32 (flashplayer_sa.exe)

## Current Supported Platform
 - [x] 7K7K
 - [ ] 4399
 - [x] 7road
 - [ ] 7road Classic
 - [ ] Common login based on webview2
