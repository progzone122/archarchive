use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use crate::archarchive::ENDPOINT;

fn get_elements(fragment: &scraper::Html) -> anyhow::Result<Vec<String>> {
    let mut elements: Vec<String> = vec![];

    let selector = Selector::parse("pre a")
        .map_err(|_e| anyhow!("Invalid element format."))?;

    for element in fragment.select(&selector) {
        if let Some(href) = element.attr("href") {
            let element = href[..href.len() - 1].trim();
            if element != ".." {
                elements.push(element.to_string());
            }
        }
    }

    Ok(elements)
}
pub async fn parse_years() -> anyhow::Result<Vec<String>> {
    let html: String = reqwest::get(ENDPOINT)
        .await?
        .text()
        .await?;

    let fragment = Html::parse_fragment(&html);
    let mut years: Vec<String> = get_elements(&fragment)?;

    for _ in 0..3 {
        years.pop();
    }

    Ok(years)
}
pub async fn parse_months(year: &str) -> anyhow::Result<Vec<String>> {
    let html: String = reqwest::get(&format!("{}/{}", ENDPOINT, year))
        .await?
        .text()
        .await?;

    let fragment = Html::parse_fragment(&html);
    let months: Vec<String> = get_elements(&fragment)?;

    Ok(months)
}
pub async fn parse_days(year: &str, month: &str) -> anyhow::Result<Vec<String>> {
    let html: String = reqwest::get(&format!("{}/{}/{}", ENDPOINT, year, month))
        .await?
        .text()
        .await?;

    let fragment = Html::parse_fragment(&html);
    let days: Vec<String> = get_elements(&fragment)?;

    Ok(days)
}