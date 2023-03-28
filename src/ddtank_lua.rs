use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version)]
struct Args {
    /// Lua script name(path), required!
    #[arg(short, long)]
    name: PathBuf,

    /// Username, default to environment variable DDTANK_USERNAME or "".
    #[arg(short, long)]
    username: Option<String>,

    /// Password, default to environment variable DDTANK_PASSWORD or "".
    #[arg(short, long)]
    password: Option<String>,

    /// Server ID, default to environment variable DDTANK_SERVER_ID or "".
    #[arg(short, long)]
    server_id: Option<String>,
}

trait LoadFromEnv {
    fn unwrap_or_load_from_env(self, env_name: &str) -> String;
}

impl LoadFromEnv for Option<String> {
    fn unwrap_or_load_from_env(self, env_name: &str) -> String {
        self.unwrap_or_else(|| {
            if let Ok(env_username) = std::env::var(env_name) {
                return env_username;
            }
            "".to_owned()
        })
    }
}

// ddtank-rs lua test interface.
fn main() {
    let args = Args::parse();
    let script_path = args.name;

    let username = args.username.unwrap_or_load_from_env("DDTANK_USERNAME");
    let password = args.password.unwrap_or_load_from_env("DDTANK_PASSWORD");
    let server = args.server_id.unwrap_or_load_from_env("DDTANK_SERVER_ID");

    let script = fs::read_to_string(&script_path).unwrap();
    let result = ddtank_rs::execute_strategy(&script, &username, &password, &server).unwrap();

    println!("script {:?} runs with result:", &script_path);
    println!("{}", result);
}
