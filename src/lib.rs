use std::{collections::HashMap, io::Read, sync::Arc};

use anyhow::{anyhow, Result};
use crypto::digest::Digest;
use reqwest::cookie::Jar;

pub mod store_engine;

#[derive(Default)]
pub struct Strategy {
    scripts: HashMap<String, String>,
}

impl Strategy {
    pub fn new(pattern: &str) -> Self {
        let mut strategy = Strategy::default();
        strategy.load(pattern);
        strategy
    }

    /// Load scripts from path that match a glob pattern.
    pub fn load(&mut self, pattern: &str) {
        for entry in glob::glob(pattern).expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
                let script = std::fs::read_to_string(path).unwrap();
                self.scripts.insert(file_name, script);
            }
        }
    }

    /// Get a vector that lists all strategy name.
    pub fn list(&self) -> Vec<String> {
        self.scripts.keys().map(|key| key.to_owned()).collect()
    }

    pub fn get(&self, name: &str) -> Result<String> {
        let script = self
            .scripts
            .get(name)
            .ok_or_else(|| anyhow!(format!("stratrgy {} do not exist", name)))?
            .to_owned();

        Ok(script)
    }
}

/// Execute a strategy by name
pub fn execute_strategy(
    script: &str,
    username: &str,
    password: &str,
    server: &str,
) -> Result<String> {
    let lua = rlua::Lua::new();
    let result = lua.context(|lua_context| -> Result<String> {
        let globals = lua_context.globals();

        let agent_constructor = lua_context.create_function(|_, ()| Ok(Agent::new()))?;
        globals.set("agent", agent_constructor)?;

        let crypto_rs = lua_context.create_table()?;
        let md5 = lua_context.create_function(|_, input: String| {
            let mut md5 = crypto::md5::Md5::new();
            md5.input_str(&input);
            Ok(md5.result_str())
        })?;

        crypto_rs.set("md5", md5)?;
        globals.set("crypto", crypto_rs)?;

        let cowv2_func =
            lua_context.create_function(|_, (url, re, title): (String, String, String)| {
                let result = get_cookie_by_cowv2(url, re, title).unwrap();
                Ok(result)
            })?;
        globals.set("get_cookie_by_cowv2", cowv2_func)?;

        lua_context.load(&script).exec()?;
        let login_function: rlua::Function = globals.get("login")?;
        let result = login_function.call::<_, String>((username, password, server))?;

        Ok(result)
    })?;

    Ok(result)
}

struct Agent {
    client: reqwest::blocking::Client,
    cookie_jar: Arc<Jar>,
}

impl Agent {
    fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1.0.3705;)"
                .parse()
                .unwrap(),
        );

        let cookie_jar: Arc<Jar> = Default::default();
        let cookie_jar1 = cookie_jar.clone();

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .cookie_provider(cookie_jar1)
            .build()
            .unwrap();

        Self { client, cookie_jar }
    }
}

impl rlua::UserData for Agent {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get", |_, agent, (url,): (String,)| {
            let response = agent.client.get(url).send().unwrap().text().unwrap();
            Ok(response)
        });

        methods.add_method("get_with", |_, agent, (url,): (String,)| {
            let response = agent.client.get(url).send().unwrap();
            let url = response.url();
            let url = format!("{}://{}/", url.scheme(), url.host().unwrap());
            let text = response.text().unwrap();
            Ok((text, url))
        });

        methods.add_method("post", |_, agent, (url, form): (String, rlua::Table)| {
            let form: std::collections::HashMap<String, String> = form
                .pairs::<String, String>()
                .into_iter()
                .map(|pair| {
                    let (k, v) = pair.unwrap();
                    (k, v)
                })
                .collect();

            let response = agent.client.post(url).form(&form).send().unwrap();
            let response_text = response.text().unwrap();

            Ok(response_text)
        });

        methods.add_method(
            "load_cookie",
            |_, agent, (url, cookies): (String, String)| {
                let url: reqwest::Url = url.parse().unwrap();

                let jar1 = agent.cookie_jar.clone();
                for cookie in cookies.split(';').map(|x| x.trim()) {
                    jar1.add_cookie_str(cookie, &url);
                }

                Ok("".to_owned())
            },
        );
    }
}

fn get_cookie_by_cowv2(url: String, re: String, title: String) -> Result<String> {
    let mut cowv2 = std::process::Command::new("cowv2")
        .args(["-u", &url, "-r", &re, "-t", &title])
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let status = cowv2.wait()?;
    if status.success() {
        let mut stdout = cowv2.stdout.take().unwrap();
        let mut cookies = String::new();
        stdout.read_to_string(&mut cookies)?;
        Ok(cookies)
    } else {
        Err(anyhow::anyhow!("cowv2 exit with no cookie"))
    }
}
