import * as Storage from "@storage"; 
import * as Env from "@env";
import * as Sciter from "@sciter";

const initDb = storage => { 
  storage.root = { 
    version: 1, 
    accountsByDate: storage.createIndex("date", false), // list of accounts indexed by date of creation
    accountsById:   storage.createIndex("string", true) // list of accounts indexed by their UID
  }
  return storage.root; 
}

let storage = Storage.open(Env.path("documents") +"/ddtank-account.db");
// var storage = Storage.open("./ddtank-account.db");
let root = storage.root || initDb(storage); // get root data object or initialize DB

document.on("beforeunload", () => {
  root = undefined;
  storage.close();
  storage = undefined;
});

const add_account = (username, password, platform, server, nickname = undefined, date = undefined, id = undefined) => {
  const account = {
    id: id || Sciter.uuid(),
    username: username,
    password: password,
    platform: platform,
    server: server,
    nickname: nickname,
    date: date || new Date(),
  }
  
  let root = storage.root;
  root.accountsByDate.set(account.date, account); 
  root.accountsById.set(account.id, account);

  storage.commit();
}

const get_account = id => root.accountsById.get(id);
const get_all_accounts = () => {
  let account_list = []
  for (let account of root.accountsByDate) {
    let {id, username, password, platform, server, nickname, ...others} = account;
    account_list.push({id, username, password, platform, server, nickname});
  }
  return account_list;
}

const delete_account = id => {
  let account = get_account(id);
  let result = root.accountsByDate.delete(account.date, account);
  result = root.accountsById.delete(account.id);
  storage.commit();
}

const replace_account = (id, obj) => {
  let account = get_account(id);
  root.accountsById.set(id, obj);
  root.accountsByDate.set(account.date, obj, true);
}


export {
  get_account,
  get_all_accounts,
  add_account,
  replace_account,
  delete_account
}
