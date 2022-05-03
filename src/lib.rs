use anyhow::{anyhow, Result};
use select::{document::Document, predicate::Attr};

pub fn login(platform: &str, username: &str, password: &str, server: &str) -> Result<String>  {
    match platform.trim().to_lowercase().as_str() {
        "7k7k" => login_7k7k(username, password, server),
        _ => Err(anyhow!("such platform is unsupported."))
    }
}

pub fn login_7k7k(username: &str, password: &str, server: &str) -> Result<String> {
    let agent = ureq::AgentBuilder::new()
        .user_agent("Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; .NET CLR 1.0.3705;)")
        .build();
    
    agent.post("http://web.7k7k.com/source/Post.php")
        .send_form(&[
            ("username", username),
            ("password", password),
            ("action", "login")
        ])?;
    
    let response = agent.get(format!("http://web.7k7k.com/games/togame.php?target=ddt_7&server_id={}", server).as_str())
        .call()?
        .into_string()?;

    let document = Document::from(response.as_str());
    let url = document.find(Attr("id", "url")).next()
        .ok_or_else(|| anyhow!("failed to get game url. please check user data."))?
        .attr("value").ok_or_else(|| anyhow!("failed to get game url. the program may not work. check update."))?;

    let response = agent.get(url).call()?;

    let flash_url = url::Url::parse(response.get_url())?;
    let response_text = response.into_string()?;

    let document = Document::from(response_text.as_str());
    let flash_path = document.find(Attr("name", "movie")).next()
        .ok_or_else(|| anyhow!("failed to get flash path. the program may not work. check update."))?
        .attr("value").ok_or_else(|| anyhow!("failed to get flash path. the program may not work. check update."))?;

    let flash_url = flash_url.join(format!("../{}", flash_path).as_str())?
        .to_string();

    Ok(flash_url)
}