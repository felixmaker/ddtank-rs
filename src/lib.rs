use std::collections::HashMap;

use anyhow::{anyhow, Result};
use crypto::digest::Digest;

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
        let userdata = lua_context.create_table()?;
        userdata.set("username", username)?;
        userdata.set("password", password)?;
        userdata.set("server_id", server)?;

        let globals = lua_context.globals();
        globals.set("userdata", userdata)?;

        let agent_constructor = lua_context.create_function(|_, ()| Ok(Agent::new()))?;
        globals.set("agent", agent_constructor)?;

        let crypto_rs = lua_context.create_table()?;
        let md5 = lua_context.create_function(|_, (input,): (String,)| {
            let mut md5 = crypto::md5::Md5::new();
            md5.input_str(&input);
            Ok(md5.result_str())
        })?;

        crypto_rs.set("md5", md5)?;

        globals.set("crypto", crypto_rs)?;

        let result = lua_context.load(&script).eval::<String>()?;

        Ok(result)
    })?;

    Ok(result)
}

struct Agent {
    client: reqwest::blocking::Client,
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

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();

        Self { client }
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
    }
}
