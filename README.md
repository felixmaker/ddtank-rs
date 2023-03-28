# DDTANK-rs
DDTANK-rs is an easy-to-use ddtank login tool. It supports Linux, Windows and MacOS.
It's recommanded to play ddtank in web browsers. However you can play it in an standalone 
flashplayer (maybe available at [web archive](https://archive.org/details/standaloneflashplayers)). 
Windows user can downlaod a standalone flashplayer from [flash.cn](https://www.flash.cn/support/debug-downloads)

## Use Lua script to create login strategy

In `./scripts` folder, create a lua script named `xxx.lua`, in which you need to 
create a function: `function login(username, password, server)`. 

You can use following Lua script API to create login game strategy:
 - _G.agent (get, post, get_with)
 - _G.crypto (md5)

See [7k7k.lua](./scripts/7k7k.lua) for example.

## Use ddtank-lua command line tool to test script (beta)

```powershell
ddtank-lua -n ./script/7k7k.lua -u $env:DDTANK_USERNAME -p $env:DDTANK_PASSWORD -s $env:DDTANK_SERVER_ID
```

Use `ddtank-lua --help` to see detailed usage.

## Requirements
 - scite-js-sdk 5.0.2.7 (sciter.dll)
 - Standalone flash player 32 (flashplayer_sa.exe)

## Current Supported Platform
 - [x] 7K7K
 - [ ] 4399
 - [x] 7road
 - [x] 7road Classic
 - [ ] Common login based on webview2
