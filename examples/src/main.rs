use regex::Regex;
use reqwest::Client;
use skim_navi::Navi;
use std::io::Error;

async fn list(input: String) -> Result<String, Error> {
    let res = Client::new()
        .get(input)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(res)
}

pub async fn get_links(input: String) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    let res = list(input).await.unwrap();
    let lines: Vec<&str> = res.split('\n').collect();

    for line in lines {
        let re = Regex::new(r#".*href="/(.+?)".*"#).unwrap();
        let caps = re.captures(line);
        if caps.is_some() {
            result.push(caps.unwrap().get(1).unwrap().as_str().to_string())
        }
    }
    result.push("..".to_string());

    result.sort();

    Ok(result)
}

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() {
    tokio::spawn(async {
        let ip = [0, 0, 0, 0];
        let footer = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        warpy::server::run("/tmp".to_string(), ip, footer, None, false)
            .await
            .unwrap();
    });

    let result = Navi::run("http://localhost:8080", get_links).await;
    println!("{:?}", result);
}
