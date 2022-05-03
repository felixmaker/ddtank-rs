#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

use sciter::*;

#[derive(Default)]
struct DDTankHandler;

impl DDTankHandler {
    fn login(&mut self, platform: String, username: String, password: String, server: String, done: sciter::Value) -> bool {
        use std::thread;
		thread::spawn(move || {
            let response = ddtank_rs::login(platform.as_str(), username.as_str(), password.as_str(), server.as_str());
            let data = match response {
                Ok(login_url) => {
                    println!("logined: {}", login_url);
                    Value::from(login_url)
                },
                Err(err) => {
                    println!("err: {}", err);
                    Value::from(format!("error: {}", err))
                }
            };
			// call `onDone` callback
			done.call(None, &make_args!(data), None).unwrap();
		});
        true
    }
}

impl sciter::EventHandler for DDTankHandler {
    sciter::dispatch_script_call! {
        fn login(String, String, String, String, Value);
    }
}

fn main() {
    let resources = include_bytes!("ui.rc");
    let ddtank_handler = DDTankHandler::default();

    let mut frame = sciter::Window::new();
    frame.archive_handler(resources).expect("Invalid archive");
    frame.event_handler(ddtank_handler);    
    frame.load_file("this://app/index.htm");
    frame.run_app();
}
