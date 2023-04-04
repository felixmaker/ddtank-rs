const add_account = (username, password, strategy, server, nickname = undefined) => {
  const account = {
    username: username,
    password: password,
    strategy: strategy,
    server: server,
    nickname: nickname
  }
  return Window.this.xcall("database_add", account);
}

const get_account = id => Window.this.xcall("database_get", id);
const get_all_accounts = () => {
  // let account_list = []
  // for (let account of root.accountsByDate) {
  //   let { id, username, password, strategy, server, nickname, ...others } = account;
  //   account_list.push({ id, username, password, strategy, server, nickname });
  // }
  // return account_list;
  let account = Window.this.xcall("database_get_all");
  return account
}

const delete_account = id => {
  // let account = get_account(id);
  // let result = root.accountsByDate.delete(account.date, account);
  // result = root.accountsById.delete(account.id);
  // storage.commit();
  return Window.this.xcall("database_delete", id);
}

const replace_account = (id, obj) => {
  return Window.this.xcall("database_replace", id, obj);
}


export {
  get_account,
  get_all_accounts,
  add_account,
  replace_account,
  delete_account
}
