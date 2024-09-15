use reqwest::Error;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Fetch the HTML content from the URL
    let body = reqwest::get("https://chat.openai.com/c/17629414-e9b3-400a-87d3-33125a5f5b0f").await?.text().await?;

    // Parse the HTML document
    let fragment = Html::parse_document(&body);

    // Use a CSS selector to find elements with the class "quote"
    let quotes = Selector::parse(".markdown").unwrap();

    // Iterate over the selected elements and print their text content
    for quote in fragment.select(&quotes) {
        let quote_text = quote.text().collect::<Vec<_>>().join(" ");
        println!("{}", quote_text);
    }

    Ok(())
}
