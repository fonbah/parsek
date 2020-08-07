use hyper::{Client, Body, Method, Request, Uri, body::Buf};
use hyper_tls::HttpsConnector;

pub async fn fetch(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let splited: Vec<&str> = url.split("/").collect();
   
    let req = Request::builder()
    .method(Method::GET)
    .uri(url)
    .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
    //.header("Accept-Encoding", "gzip, deflate, br")
    .header("Accept-Language", "ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3")
    .header("Cache-Control", "max-age=0")
    .header("Connection", "keep-alive")
    .header("Host", splited[2])
    .header("User-Agent", "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:79.0) Gecko/20100101 Firefox/79.0")
    .body(Body::from(""));
    
    match req {
        Ok(req) => {
            let resp = client.request(req).await?;
            let mut body = hyper::body::aggregate(resp).await?;
            Ok(body.to_bytes().to_vec())
        },
        Err(e) => Ok(Vec::new())
    }
}
