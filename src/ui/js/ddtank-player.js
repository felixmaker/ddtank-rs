import * as sys from "@sys";
import * as env from "@env";

const player_exists = sys.fs.$readdir("ddtank-player") ? true : false;
const flashplayer = env.PLATFORM == "Windows" ? "./flashplayer_sa.exe" : "./flashplayer";

export const play = (title = "弹弹堂", url = "") => {
    if (player_exists) {
        env.exec("./ddtank-player/ddtank-player", `--title=${title}`, `--url=${url}`)
        return;
    }
    env.exec(flashplayer, url)
}
