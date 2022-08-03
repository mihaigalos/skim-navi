use regex::Regex;
use reqwest::Client;
use std::io::Error;

async fn list(input: &str) -> Result<String, Error> {
    let res = Client::new()
        .get(input)
        .send()
        .await
        .or(Err(format!("Failed to GET from {}", &input)))
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(res)
}

pub async fn get_links(input: &str) -> Result<Vec<String>, Error> {
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

    let _ = tokio::spawn(async {
        let links = get_links("http://localhost:8080").await.unwrap();
        println!("{:?}", links);
    })
    .await;
}
