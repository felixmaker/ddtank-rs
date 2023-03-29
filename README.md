# DDTANK-rs
DDTANK-rs is an easy-to-use ddtank login tool. It supports Linux, Windows and MacOS.
It's recommanded to play ddtank in web browsers. However you can play it in an standalone 
flashplayer (maybe available at [web archive](https://archive.org/details/standaloneflashplayers)). 
Windows user can downlaod a standalone flashplayer from [flash.cn](https://www.flash.cn/support/debug-downloads)

## Use Lua script to create login strategy

In `./scripts` folder, create a lua script named `xxx.lua`, in which you need to 
create a function: `function login(username, password, server)`. 

You can use following Lua script API to create login game strategy:
 - _G.agent (get, post, get_with, load_cookie)
 - _G.crypto (md5)
 - _G.get_cookie_by_cowv2

See `./scripts` folder.

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
 - [x] 4399 (beta, based on webview2)
 - [x] 7road
 - [x] 7road Classic
 - [x] Common login based on webview2 (beta)
