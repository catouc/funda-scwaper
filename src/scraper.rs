use std::collections::HashSet;
use select::document::Document;
use select::predicate::Name;

pub mod makelaar;
pub mod detail_page;

pub struct Scraper {
    pub client: reqwest::blocking::Client,
    start_url: String,
}

impl Scraper {
    // https://www.funda.nl/zoeken/koop?selected_area=%5B%22amsterdam%22%5D&price=%22150000-800000%22&object_type=%5B%22apartment%22%5D&availability=%5B%22available%22%5D&floor_area=%2250-%22
    pub fn crawl(&self) {

        let response = self.client
            .get(self.start_url.as_str())
            .version(reqwest::Version::HTTP_2)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let links = Self::get_links_from_search_html(response);
        
        for link in links.iter() {
            let detail_page = self.client
                .get(link)
                .version(reqwest::Version::HTTP_2)
                .send()
                .unwrap()
                .text()
                .unwrap();
            
            println!("{:?}", link);
            /*
            let makelaar_map = makelaar::create_makelaar_map_from_html(&detail_page);
            for (link, name) in makelaar_map.into_iter() {
                println!("{},{}", link, name)
            };
            */

            detail_page::get_details_from_html(&detail_page);
        }
    }

    fn get_links_from_search_html(html: String) -> HashSet<String> {
        let mut links: HashSet<String> = HashSet::new();
        Document::from(html.as_str())
            .find(Name("a"))
            .filter(|n| Self::is_listing_url(n))
            .for_each(|x| {
                links.insert(String::from(x.attr("href").unwrap()));
            });

        links
    }

    fn is_listing_url(node: &select::node::Node) -> bool {
        match node.attr("href") {
            Some(str) => {
                if str == "https://www.funda.nl/koop/heel-nederland/" {
                    return false;
                }

                if str.starts_with("https://www.funda.nl/details/koop") {
                    return true;
                }

                if str.starts_with("https://www.funda.nl/koop") {
                    return true;
                } else {
                    return false;
                }
            }
            None => false
        }
    }
}

pub fn new(start_url: &str, user_agent: &str) -> Scraper {
    let client = reqwest::blocking::Client::builder()
        .http2_prior_knowledge()
        .user_agent(user_agent)
        .build()
        .unwrap();

    Scraper {
        client,
        start_url: String::from(start_url),
    }
}


