use crate::config::WikiConfig;
use reqwest::Error;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
struct WikipediaResponse {
    query: Query,
    #[serde(rename = "continue")]
    continue_data: Option<ContinueData>,
}

#[derive(Debug, Deserialize)]
struct Query {
    pages: std::collections::HashMap<String, Page>,
}

#[derive(Debug, Deserialize)]
struct Page {
    title: String,
    links: Option<Vec<Link>>,
    linkshere: Option<Vec<LinkHere>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub title: String,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
impl Display for LinkHere {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LinkHere {
    pub title: String,
}

#[derive(Debug, Deserialize)]
struct ContinueData {
    lhcontinue: Option<String>,
    plcontinue: Option<String>,
}

pub async fn get_links(title: &str, config: &WikiConfig) -> Result<Vec<Link>, Error> {
    let url = format!(
        "https://{}.wikipedia.org/w/api.php?action=query&titles={}&prop=links&format=json&pllimit={}",
        config.language, title, config.pllimit
    );

    let response = reqwest::get(&url)
        .await?
        .json::<WikipediaResponse>()
        .await?;

    if let Some(pages) = response.query.pages.values().next() {
        if let Some(links) = &pages.links {
            return Ok(links.clone());
        }
    }

    Ok(Vec::new())
}

pub async fn get_links_here(title: &str, config: &WikiConfig) -> Result<Vec<LinkHere>, Error> {
    let url = format!(
        "https://{}.wikipedia.org/w/api.php?action=query&titles={}&prop=linkshere&format=json&lhlimit={}",
        config.language, title, config.pllimit
    );

    let response = reqwest::get(&url)
        .await?
        .json::<WikipediaResponse>()
        .await?;

    if let Some(pages) = response.query.pages.values().next() {
        if let Some(links) = &pages.linkshere {
            return Ok(links.clone());
        }
    }

    Ok(Vec::new())
}
