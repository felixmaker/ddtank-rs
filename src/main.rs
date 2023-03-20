#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sciter::{make_args, Value, Window};
use std::thread;

#[derive(Default)]
struct DDTankHandler {
    strategy: ddtank_rs::Strategy,
}

impl DDTankHandler {
    fn new() -> Self {
        let strategy = ddtank_rs::Strategy::new("./scripts/*.lua");
        Self { strategy }
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
}

impl sciter::EventHandler for DDTankHandler {
    sciter::dispatch_script_call! {
        fn login(String, String, String, String, Value);
        fn get_all_strategy();
        fn play_flash(String);
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
