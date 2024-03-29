import * as account_db from "./accountdb.js"

const { signal } = Reactor;
const accounts = signal(account_db.get_all_accounts());

export const App = function () {
    return (
        <div>
            <header>
                <button onclick={() => show_add_account_dialog()}>👩‍🚒 添加账户</button>
                <button onclick={() => accounts.value = account_db.get_all_accounts()}>🧭 刷新</button>
            </header>
            <main>
                <div id="account-list">
                    {Object.entries(accounts.value).map(([account_id, account]) => {
                        return <div class="account"
                            ondoubleclick={() => login_account(account_id)}
                            oncontextmenu={(event) => {
                                event.source = Element.create(<menu class="context">
                                    <li onclick={() => login_account(account_id)}>登录</li>
                                    <li onclick={() => show_edit_account_dialog(account_id)}>修改</li>
                                    <li onclick={() => delete_account(account_id)}>删除</li>
                                </menu>);
                                return true;
                            }}>
                            <div class="account-avator" disabled>💂</div>
                            <div class="account-detail" disabled>{account.strategy} - {account.nickname || account.username}</div>
                        </div>
                    })}
                </div>
            </main>
        </div >
    )
}

const login_account = (account_id) => {
    const account = account_db.get_account(account_id);
    const { strategy, username, password, server, nickname } = account;

    Window.this.xcall("login", strategy, username, password, server, (response) => {
        if (response.startsWith("http")) {
            Window.this.xcall("play_flash", response);
        }
        else {
            Window.this.modal(<error>{response}</error>);
        }
    });
}

const show_add_account_dialog = () => {
    var data = Window.this.modal({
        url: __DIR__ + "../htm/add-account.htm",
        parameters: {
            strategy_list: Window.this.xcall("get_all_strategy"),
        }
    });

    if (data != undefined) {
        let { username, password, strategy, server, nickname } = data;
        account_db.add_account(username, password, strategy, server, nickname);
    }

    accounts.value = account_db.get_all_accounts();
}

const show_edit_account_dialog = (account_id) => {
    let account = account_db.get_account(account_id);
    let data = Window.this.modal({
        url: __DIR__ + "../htm/edit-account.htm",
        parameters: {
            strategy_list: Window.this.xcall("get_all_strategy"),
            account: account
        }
    });

    if (data != undefined) {
        let { username, password, strategy, server, nickname } = data;

        account.username = username;
        account.password = password;
        account.strategy = strategy;
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
