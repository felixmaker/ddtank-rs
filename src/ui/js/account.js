export class Account extends Element {
               
    render(props) {
        let {key, platform, username, nickname, ...others} = props;
        return <div class="account" key={key}>
            <div class="account-avator">💂</div>
            <div class="account-detail">{platform} - {nickname || username}</div>
        </div>
    }

    ['on contextmenu at .account'](event) {
        event.source = Element.create(<menu class="context">
            <li>登录</li>
            <li>修改</li>
            <li>删除</li>
        </menu>);
        return true;
    }
    
    ["on click at menu.context>li"](event, menuitem) {
        let account_id = this.getAttribute("key");
        switch (menuitem.innerText) {
            case "登录":
                this.post(new Event("account-login", {bubbles: true, data: account_id}));
                break;
            case "修改":
                this.post(new Event("account-modify", {bubbles: true, data: account_id}));
                break;
            case "删除":
                this.post(new Event("account-delete", {bubbles: true, data: account_id}));
                break;
        }
    }

}
