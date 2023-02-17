#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://blog.feedspot.com/pakistan_news_rss_feeds")
        .await?
        .text()
        .await?;

    let doc = scraper::Html::parse_document(&body);

    let selector = scraper::Selector::parse("a.extdomain.ext").unwrap();
    let links = doc.select(&selector).map(|x| x.inner_html());

    links
        .zip(1..84)
        .for_each(|(item, number)| println!("{}. {}", number, item));

    Ok(())
}
