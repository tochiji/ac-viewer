use hyper::{body::HttpBody, Client};
use hyper_tls::HttpsConnector;

pub async fn fetch_html(url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = url.parse().unwrap();
    let mut res = client.get(uri).await?;

    let mut str = String::from("");
    while let Some(chunk) = res.body_mut().data().await {
        str.push_str(std::str::from_utf8(&chunk?).unwrap());
    }

    Ok(str)
}
