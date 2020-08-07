use kuchiki::traits::*;

pub fn parse (page: String) -> (String, Vec<String>) {
    let document = kuchiki::parse_html().one(page);
    let mut lines: Vec<String> = Vec::new();
    let mut title: String = String::new();
    let mut current: i8 = 0;

    for node in document.descendants() {
        if let Some(_doctype) = node.as_doctype() {
            continue;
        }

        if let Some(_comment) = node.as_comment() {
            continue;
        }

        match current {
            -1 => { current = 0; continue },
            1 => {
                if let Some(text) = node.as_text() {
                    title = text.borrow().trim().to_string();
                }
                current = 0;
                continue;
            },
            0 => (),
            _ => current = 0,
        }

        if let Some(element) = node.as_element() {
            match element.name.local.trim() {
                "script"|"noscript"|"style"|"source" => { current = -1; continue; },
                "title" => { current = 1; lines.clear(); continue; },
                _ => continue,
            }
        }

        if let Some(text) = node.as_text() {
            if !text.borrow().trim().is_empty() {
                if title.len() > 10 && lines.len() > 0 {
                    if title.starts_with(text.borrow().trim()) {
                        if let Some(t) = lines.get(0) {
                            if !t.starts_with(text.borrow().trim()) {
                                lines.clear();
                            }
                        }
                    }
                }
                lines.push(text.borrow().trim().to_string());
            }
        }
    }
    
    let ttl = match lines.get(0) {
        Some(lines_first) => { if title.len() > 1 && title.starts_with(lines_first) { lines_first.to_string() } else { title } },
        None => String::new(),
    };
    
    let result = match lines.len() > 0 {
        true => lines[1..].to_vec(),
        false => lines,
    };
    
    (ttl, result)
}
