use std::env;
//use crate::scraper::makelaar;

pub mod scraper;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    let scraper = scraper::new(url, "Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0");

    scraper.crawl();
/*
    let response = scraper.client
        .get(url)
        .version(reqwest::Version::HTTP_2)
        .send()
        .unwrap()
        .text()
        .unwrap();
    
*/

}
