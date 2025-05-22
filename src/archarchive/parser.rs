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
fn build_url(year: &str, month: Option<&str>, day: Option<&str>) -> String {
    match (month, day) {
        (Some(m), Some(d)) => format!("{}/{}/{}/", ENDPOINT, year, m, d),
        (Some(m), None)    => format!("{}/{}/", ENDPOINT, year, m),
        (None, None)       => format!("{}/", ENDPOINT),
        _ => unreachable!()
    }
}
pub async fn parse_years() -> anyhow::Result<Vec<String>> {
    let html: String = reqwest::get(ENDPOINT)
        .await?
        .text()
        .await?;

    let fragment = Html::parse_fragment(&html);
    let mut years: Vec<String> = get_elements(&fragment)?;

    const FOOTER_LINKS_COUNT: usize = 3;
    for _ in 0..FOOTER_LINKS_COUNT {
        years.pop();
    }

    Ok(years)
}
pub async fn parse_months(year: &str) -> anyhow::Result<Vec<String>> {
    let html: String = reqwest::get(&build_url(year, Some(month), None)).await?.text().await?;
    let fragment = Html::parse_fragment(&html);
    let months: Vec<String> = get_elements(&fragment)?;

    Ok(months)
}
pub async fn parse_days(year: &str, month: &str) -> anyhow::Result<Vec<String>> {
    let html: String = reqwest::get(&build_url(year, Some(month), Some(day))).await?.text().await?;
    let fragment = Html::parse_fragment(&html);
    let days: Vec<String> = get_elements(&fragment)?;

    Ok(days)
}
