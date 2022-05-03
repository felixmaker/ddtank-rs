import * as Env from "@env";
import * as DB from "./accountdb.js"
import {Account} from "./account.js";

export class App extends Element {

    // note: in reality that should be virtual list.

    componentDidMount() {
        this.onGlobalEvent("refresh-needed", () => {
            this.componentUpdate();
        });
    }

    render() {

        var list = [];

        for (let account of DB.Account.all()) {
            let {id, username, password, platform, server, nickname, ...others} = account;
            list.push(<Account key={id} platform={platform} username={username} nickname={nickname} />);
        }

        return <div>
            <header>
                <button#add-account>👩‍🚒 添加账户</button>
                <button#refresh>🧭 刷新</button>
            </header>
            <main>
                <div#account-list>{list}</div>
            </main>
        </div>
    }

    ['on click at button#add-account'](event) {
        var data = Window.this.modal({
            url: __DIR__ + "../htm/add-account.htm"
        });

        if (data != undefined) {
            let {username, password, platform, server, nickname} = data;
            new DB.Account(username, password, platform, server, nickname);
        }
    }

    ['on click at button#fresh'](event) {
        this.componentUpdate();
    }

    ['on account-login'](event) {
        let account_id = event.data.account_id;
        let account = DB.Account.getById(account_id);
        let {platform, username, password, server, ...others} = account;

        Window.this.xcall("login", platform, username, password, server, (response)=>{
            if (response.startsWith("http")) {                                
                Env.exec("flashplayer_sa.exe", response)
            }
            if (response.startsWith("error")) {
                Window.this.modal(<error>{response.substring(6)}</error>);
            }
        });
    }

    ['on account-modify'](event) {
        let account_id = event.data.account_id;
        let account = DB.Account.getById(account_id);
        let data = Window.this.modal({
            url: __DIR__ + "../htm/edit-account.htm",
            parameters: account
        });

        if (data != undefined) {
            let {username, password, platform, server, nickname, ...others} = data;

            account.username = username;
            account.password = password;
            account.platform = platform;
            account.server = server;
            account.nickname = nickname;

            DB.Account.replaceById(account_id, account);
        }
    }
    
    ['on account-delete'](event) {
        DB.Account.deleteById(event.data);
    }
}

