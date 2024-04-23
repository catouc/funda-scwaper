use select::document::Document;
use select::predicate::Name;

#[derive(Debug)]
pub struct Makelaar {
    name: String,
    url: String,
}

pub fn create_makelaar_map_from_doc(doc: &Document) -> Makelaar {
    let mut makelaar: Makelaar = Makelaar{name: "".to_string(), url: "".to_string()};
    doc
        .find(Name("a"))
        .filter(|n| is_makelaar_href(n))
        .filter(|n| is_makelaar_node(n))
        .for_each(|x| {
            makelaar.name = String::from(x.attr("title").unwrap());
            makelaar.url = String::from(x.attr("href").unwrap());
        });
    makelaar
}

fn is_makelaar_href(node: &select::node::Node) -> bool {
    match node.attr("href") {
        Some(str) => {
            if str.contains("makelaar") {
                true
            } else {
                false
            }
        }
        None => return false
    }
}

fn is_makelaar_node(node: &select::node::Node) -> bool {
    match node.attr("title") {
        Some(_str) => return true,
        None => return false
    }
}

