fn main() {
    let url = "https://typescript.tv/errors/";
    let response = reqwest::blocking::get(url);

    let content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&content);

    let selector = scraper::Selector::parse("h2, blockquote").unwrap();

    for element in document.select(&selector) {
        println!("{}", element.text().collect::<String>());
    }
}
