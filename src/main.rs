#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ddtank_rs::{store_engine::StoreEngine, userinfo::UserInfo};

use sciter::{make_args, Value, Window};
use std::thread;

struct DDTankHandler {
    strategy: ddtank_rs::Strategy,
    db: StoreEngine,
}

impl DDTankHandler {
    fn new() -> Self {
        let strategy = ddtank_rs::Strategy::new("./scripts/*.lua");
        let db = StoreEngine::create("userdata.redb").unwrap();
        Self { strategy, db }
    }

    fn login(
        &mut self,
        strategy: String,
        username: String,
        password: String,
        server: String,
        done_callback: Value,
    ) -> bool {
        let script = self.strategy.get(&strategy).unwrap();
        thread::spawn(move || {
            let result = ddtank_rs::execute_strategy(&script, &username, &password, &server);
            let result = match result {
                Ok(url) => url,
                Err(err) => format!("error{}", err.to_string()),
            };
            done_callback.call(None, &make_args!(result), None).unwrap();
        });
        true
    }

    fn get_all_strategy(&self) -> Value {
        let strategy_list = self.strategy.list();
        Value::from_iter(strategy_list)
    }

    fn play_flash(&self, url: String) -> anyhow::Result<()> {
        let flashplayer = if cfg!(target_os = "windows") {
            "./flashplayer_sa.exe"
        } else {
            "./flashplayer"
        };

        std::process::Command::new(flashplayer).arg(url).output()?;
        Ok(())
    }

    fn database_get(&self, user_id: String) -> anyhow::Result<Value> {
        let uuid = uuid::Uuid::parse_str(&user_id)?;
        let user = self
            .db
            .get_user(&uuid)
            .ok_or_else(|| anyhow::anyhow!("no such user id"))?;
        let user_value = sciter_serde::to_value(&user)?;
        Ok(user_value)
    }

    fn database_get_all(&self) -> anyhow::Result<Value> {
        let users: std::collections::HashMap<String, UserInfo> = self
            .db
            .users()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        let users_json = sciter_serde::to_value(&users)?;
        Ok(users_json)
    }

    fn database_add(&mut self, user: Value) -> anyhow::Result<()> {
        let uuid = uuid::Uuid::new_v4();
        let mut user = user.clone();
        user.isolate();
        let user: UserInfo = sciter_serde::from_value(&user)?;
        self.db.insert(&uuid, &user)?;
        Ok(())
    }

    fn database_replace(&mut self, uuid: String, user: Value) -> anyhow::Result<()> {
        let uuid = uuid::Uuid::parse_str(&uuid)?;
        let mut user = user.clone();
        user.isolate();
        let user: UserInfo = sciter_serde::from_value(&user).unwrap();
        self.db.insert(&uuid, &user)?;
        Ok(())
    }

    fn database_delete(&mut self, uuid: String) -> anyhow::Result<()> {
        let uuid = uuid::Uuid::parse_str(&uuid)?;
        self.db.remove(&uuid)?;
        Ok(())
    }
}

impl sciter::EventHandler for DDTankHandler {
    sciter::dispatch_script_call! {
        fn login(String, String, String, String, Value);
        fn get_all_strategy();
        fn play_flash(String);
        fn database_get(String);
        fn database_get_all();
        fn database_add(Value);
        fn database_replace(String, Value);
        fn database_delete(String);
    }
}

fn main() {
    let resources = include_bytes!("ui.rc");
    let ddtank_handler = DDTankHandler::new();

    let mut frame = Window::new();
    frame.archive_handler(resources).expect("Invalid archive");
    frame.event_handler(ddtank_handler);
    frame.load_file("this://app/index.htm");
    frame.run_app();
}
