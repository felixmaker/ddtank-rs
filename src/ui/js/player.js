import * as sys from "@sys";
import * as Env from "@env";

let ddtank_player = "flashplayer_sa.exe";

let ddtank_player_dir = sys.fs.$readdir("ddtank-player");
if (ddtank_player_dir != null) {
    ddtank_player = "ddtank-player/ddtank-player.exe"
}

export function play(title = "弹弹堂", url = "") {
    if (ddtank_player == "ddtank-player/ddtank-player.exe") {
        Env.exec(ddtank_player, `--title=${title}`, `--url=${url}`)
    }
    if (ddtank_player == "flashplayer_sa.exe") {
        Env.exec(ddtank_player, url)
    }    
}
