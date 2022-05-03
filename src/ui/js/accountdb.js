import * as Storage from "@storage"; 
import * as Env from "@env";
import * as Sciter from "@sciter";

function initDb(storage) { 
  storage.root = { 
    version: 1, 
    accountsByDate: storage.createIndex("date", false), // list of accounts indexed by date of creation
    accountsById:   storage.createIndex("string", true) // list of accounts indexed by their UID
  }
  return storage.root; 
}

var storage = Storage.open(Env.path("documents") +"/ddtank-account.db");
// var storage = Storage.open("./ddtank-account.db");
var root = storage.root || initDb(storage); // get root data object or initialize DB

document.on("beforeunload", function(){
  root = undefined;
  storage.close();
  storage = undefined;
});

export class Account {
  
  constructor(username, password, platform, server, nickname = undefined, date = undefined, id = undefined) {
    this.id = id || Sciter.uuid();
    this.date = date || new Date();

    this.username = username;
    this.password = password;
    this.platform = platform;
    this.server = server;
    this.nickname = nickname;
    
    // adding it to storage
    let root = storage.root;
    root.accountsByDate.set(this.date, this); 
    root.accountsById.set(this.id, this);

    storage.commit(); // we do manual commit here
    Window.post(new Event("refresh-needed", {data: "account-added"}))
  }

  delete() {
    let root = storage.root;
    root.accountsByDate.delete(this.date, this); // need 'this' here as index is not unique
    root.accountsById.delete(this.id);
  }

  static getById(id) {
    return storage.root.accountsById.get(id); // will fetch object from DB and do 
                                           // Object.setPrototypeOf(account, Account.prototype)
  }

  static deleteById(id) {
    let account = this.getById(id);
    let result = root.accountsByDate.delete(account.date, account);
    result = root.accountsById.delete(account.id);
    storage.commit();
    Window.post(new Event("refresh-needed", {data: "account-removed"}));
  }

  static replaceById(id, obj) {
    let root = storage.root;
    let account = this.getById(id);
    root.accountsById.set(id, obj);
    root.accountsByDate.set(account.data, obj, true);

    Window.post(new Event("refresh-needed", {data: "account-modified"}));
  }

  static all() { // in creation date order
    return root.accountsByDate;
  }
}