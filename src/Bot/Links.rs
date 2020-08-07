use kuchiki::traits::*;

pub fn parse (url: String, page: String) -> Vec<String> {

    let splited: Vec<&str> = url.split("/")
        .filter(move |chank| chank.len() > 0)
        .collect();

    let protocol = splited[0];
    let domain = splited[1];
    let min = 1;

    let document = kuchiki::parse_html().one(page);

    match document.select("a") {

        Ok(links) => {
            let res: Vec<String> = links
            .map(move |item|
                match item.attributes.borrow().get("href") {
                    Some(href) => { 
                        if href.starts_with("//") {
                            if href.len() > (domain.len() + min) && href[2..].starts_with(&domain) {
                                protocol[..].to_string() + &href.to_string()
                            } else { "".to_string() }
                        } else if href.starts_with(&protocol) && href.len() > (protocol.len() + 2 + domain.len() + min) && href[protocol.len() + 2..].starts_with(&domain) {
                            href.to_string()
                        } else if href.starts_with("/") && href.len() > min {
                            protocol[..].to_string() + "//" + &domain[..].to_string() + &href.to_string()
                        } else {
                            "".to_string()
                        }
                    },
                    None => "".to_string(),
                }
            )
            .filter(|href| href != "")
            //.filter(|href| !(href.starts_with("data:") || href.starts_with("mailto:") || href.starts_with("javascript:")))
            .map(move |href| {
                match href.find("?") {
                    Some(q) => href[..q].to_string(),
                    None => href,
                }
            })
            .map(move |href| {
                match href.find("#") {
                    Some(a) => href[..a].to_string(),
                    None => href,
                }
            })
            .collect();
            res
        },
        _ => Vec::new()
    }
}
