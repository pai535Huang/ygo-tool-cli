use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

fn get_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| reqwest::Client::new())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardText {
    pub types: Option<String>,
    pub pdesc: Option<String>,
    pub desc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardData {
    pub atk: Option<i32>,
    pub def: Option<i32>,
    pub level: Option<i32>,
    pub attribute: Option<i32>,
    pub race: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchItem {
    pub cid: Option<i64>,
    pub id: i64,
    pub cn_name: Option<String>,
    pub sc_name: Option<String>,
    pub jp_name: Option<String>,
    pub en_name: Option<String>,
    pub text: Option<CardText>,
    pub data: Option<CardData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub result: Option<Vec<SearchItem>>,
    pub next: Option<i32>,
}

pub async fn search_card(query: &str) -> Result<Option<SearchItem>, reqwest::Error> {
    let url = "https://ygocdb.com/api/v0/";
    let res = get_client()
        .get(url)
        .query(&[("search", query)])
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;
    if let Some(mut results) = res.result {
        if !results.is_empty() {
            return Ok(Some(results.remove(0)));
        }
    }
    Ok(None)
}

pub async fn search_cards(query: &str) -> Result<Vec<SearchItem>, reqwest::Error> {
    let url = "https://ygocdb.com/api/v0/";
    let res = get_client()
        .get(url)
        .query(&[("search", query)])
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;
    if let Some(results) = res.result {
        return Ok(results);
    }
    Ok(Vec::new())
}

pub async fn download_image(id: i64) -> Result<(String, Vec<u8>), reqwest::Error> {
    let sc_url = format!("https://cdn.233.momobako.com/ygoimg/sc/{}.webp", id);
    let sc_resp = get_client().get(&sc_url).send().await?;
    
    if sc_resp.status().is_success() {
        let bytes = sc_resp.bytes().await?;
        return Ok((sc_url, bytes.to_vec()));
    }
    
    let jp_url = format!("https://cdn.233.momobako.com/ygoimg/jp/{}.webp", id);
    let jp_resp = get_client().get(&jp_url).send().await?;
    let bytes = jp_resp.error_for_status()?.bytes().await?;
    Ok((jp_url, bytes.to_vec()))
}
