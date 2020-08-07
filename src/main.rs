use tokio::net::TcpListener;
use tokio::task;
use tokio::prelude::*;
use url::{Url, ParseError};
use serde_json;
use std::collections::HashSet;

mod Bot;
use Bot::{Client, Links, Content, Domain};

async fn parse_domain(url: String, page: String) -> Result<(Vec<[String; 3]>), Box<dyn std::error::Error>> {
    let domain_links: HashSet<String> = Links::parse(url, page).into_iter().collect();

    let handles: Vec<task::JoinHandle<_>> = domain_links.into_iter().map(move |lnk| {

        let handle = tokio::spawn(async move {
            if let Ok(url) = Url::parse(&lnk) {
                let page_vec = match Client::fetch(url.as_str()).await {
                    Ok(page_vec) => page_vec,
                    Err(e) => Vec::new(),
                };
                let (ttl, lines) = match String::from_utf8(page_vec) {
                    Ok(page_string) => Content::parse(page_string),
                    Err(e) => (String::new(), Vec::new()),
                };
                (url.as_str().to_string(), ttl, lines)
            } else {
                (String::new(), String::new(), Vec::new())
            }
        });
        handle
    }).collect();
    
    let mut urls_ttls: Vec<(String, String)> = Vec::new();
    let mut handles_result: Vec<Vec<String>> = Vec::new();
    
    for handle in handles.into_iter() {
       async {
           if let Ok((url, ttl, lines)) = handle.await {
               urls_ttls.push((url, ttl));
               handles_result.push(lines);
           }
       }.await
    }
    
    let lines_results = Domain::parse(handles_result);

    let results: Vec<[String; 3]> = urls_ttls.into_iter().zip(lines_results.into_iter()).map(move |((url, ttl), page)| {
        [url, ttl, page]
    }).collect();
    
    Ok(results)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let default_port = "6042";

    let p = match args.first() {
        Some(p) if p == "-p" || p == "--port" => { if let Some(p_val) = args.get(1) { p_val } else { default_port } },
        _ => default_port,
    };

    let mut listener = TcpListener::bind(format!("127.0.0.1:{}", p)).await?;

    println!("listening on {}", p);

    loop {
        let ( mut socket, _ ) = listener.accept().await?;

        tokio::spawn( async move {
            let mut buf = [0; 512];

            let n = match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => { println!("failed to read from socket, err {:?}", e); return; },
            };

            if let Ok(cmd) = String::from_utf8(buf[..n].to_vec()) {
                let cmd_splitted: Vec<&str> = cmd.trim().split_whitespace().collect();
                let link = match cmd_splitted.get(0) {
                    Some(link) => String::from(*link),
                    None => String::new(),
                };
                let param = match cmd_splitted.get(1) {
                    Some(param) => *param,
                    None => "",
                };
                if let Ok(url) = Url::parse(link.trim()) {
                    let page_vec = match Client::fetch(url.as_str()).await {
                        Ok(page_vec) => page_vec,
                        Err(e) => Vec::new(),
                    };
                    if let Ok(page_string) = String::from_utf8(page_vec) {
                        match param {
                            "-c" => if let Ok(cnt_json) = serde_json::to_string(&Content::parse(page_string)) {
                                         if let Err(e) = socket.write_all(&cnt_json.into_bytes()).await {
                                             println!("failed to write to socket, err {:?}", e); return;
                                         }
                                     },
                            "-d" => {
                                let results = match parse_domain(url.as_str().to_string(), page_string).await {
                                    Ok(results) => results,
                                    Err(e) => Vec::new(),
                                };
                                let results_json = match serde_json::to_string(&results) {
                                    Ok(results_json) => results_json,
                                    Err(e) => String::new(),
                                };
                                if let Err(e) = socket.write_all(&results_json.into_bytes()).await {
                                    println!("failed to write to socket, err {:?}", e); return;
                                }
                            },
                            _ => if let Ok(lnks_json) = serde_json::to_string(&Links::parse(url.as_str().to_string(), page_string)) {
                                     if let Err(e) = socket.write_all(&lnks_json.into_bytes()).await {
                                         println!("failed to write to socket, err {:?}", e); return;
                                     }
                                 },
                        }
                    }
                }
            }
        });
    }
}
