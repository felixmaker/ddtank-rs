import * as ddtank_player from "./ddtank-player.js";
import * as account_db from "./accountdb.js"

const { signal } = Reactor;
const accounts = signal(account_db.get_all_accounts());

export const App = () => <div>
    <header>
        <button onclick={() => show_add_account_dialog()}>👩‍🚒 添加账户</button>
        <button onclick={() => accounts.value = account_db.get_all_accounts()}>🧭 刷新</button>
    </header>
    <main>
        <div#account-list>
        { accounts.value.map((account) => <div class="account"
            ondoubleclick={() => login_account(account.id)} 
            oncontextmenu={(event) => {
                event.source = Element.create(<menu class="context">
                    <li onclick={() => login_account(account.id)}>登录</li>
                    <li onclick={() => show_edit_account_dialog(account.id)}>修改</li>
                    <li onclick={() => delete_account(account.id)}>删除</li>
                </menu>);
                return true;
            }}>
                <div class="account-avator">💂</div>
                <div class="account-detail">{account.platform} - {account.nickname || account.username}</div>
            </div>
        ) }
        </div>
    </main>
</div>

const login_account = (account_id) => {
    const account = account_db.get_account(account_id);
    const {platform, username, password, server, nickname} = account;

    Window.this.xcall("login", platform, username, password, server, (response)=>{
        if (response.startsWith("http")) {
            ddtank_player.play(`${platform} - ${nickname || username}`, response);
        }
        if (response.startsWith("error")) {
            Window.this.modal(<error>{response.substring(6)}</error>);
        }
    });
}

const show_add_account_dialog = () => {
    var data = Window.this.modal({
        url: __DIR__ + "../htm/add-account.htm"
    });

    if (data != undefined) {
        let {username, password, platform, server, nickname} = data;
        account_db.add_account(username, password, platform, server, nickname);
    }

    accounts.value = account_db.get_all_accounts();
}

const show_edit_account_dialog = (account_id) => {
    let account = account_db.get_account(account_id);
    let data = Window.this.modal({
        url: __DIR__ + "../htm/edit-account.htm",
        parameters: account
    });

    if (data != undefined) {
        let {username, password, platform, server, nickname} = data;

        account.username = username;
        account.password = password;
        account.platform = platform;
        account.server = server;
        account.nickname = nickname;

        account_db.replace_account(account_id, account);
    }

    accounts.value = account_db.get_all_accounts();
}

const delete_account = (account_id) => {
    account_db.delete_account(account_id);
    accounts.value = account_db.get_all_accounts();
}
