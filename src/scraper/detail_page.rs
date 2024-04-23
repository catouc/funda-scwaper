use crate::scraper::makelaar;

use select::document::Document;
use select::predicate::Name;
use select::predicate::Class;
use select::node::Node;

const PRICE_SELECTOR: &str = "object-header__price";
const STREET_SELECTOR: &str = "object-header__title";

#[derive(Debug)]
struct Details {
    address: String,
    price: String,
    makelaar: makelaar::Makelaar,
    /*
    link: String,
    square_meter: u32,
    number_of_bedrooms: u16,
    number_of_rooms: u16,
    energy_label: String,
    build_year: u16,
    vve_cost: u32,
    phone_number: String,
    */
}

pub fn get_details_from_html(html: &String) {
    let doc = Document::from(html.as_str());
    let price = extract_class_from_document(&doc, PRICE_SELECTOR);
    let address = extract_class_from_document(&doc, STREET_SELECTOR);
    let makelaar = makelaar::create_makelaar_map_from_doc(&doc);
    /*
    Document::from(html.as_str())
        .find(Class(PRICE_SELECTOR))
        .for_each(|x| {
            x.children()
                .for_each(|c| println!("{}", c.text()));
            }
        );
   Document::from(html.as_str())
        .find(Class("object-header__title"))
        .for_each(|x| {
            x.children()
                .for_each(|c| println!("{}", c.text()));
            }
        );
    */
    let details: Details = Details{ address, price, makelaar};
    println!("{:?}", details)
}

fn extract_class_from_document(doc: &Document, selector: &str) -> String {
    let mut result: String = "".to_string();
    doc
        .find(Class(selector))
        .for_each(|n| n.children()
                  .for_each(|c| result = c.text()
                            )
                  );
    return result;
}

/*
fn parse_price(price_str: &String) -> u32 {
    //€ 650.000 k.k.    
    //This is going to be very crude for now.
}
*/
