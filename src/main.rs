use hyper::{body::HttpBody as _, Client};
use hyper_tls::HttpsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let simple_pypi_url = "https://pypi.org/simple/";

    let package = "django";
    let _version_requirements = "3.1.1";

    let url = format!("{}{}/", simple_pypi_url, package);
    fetch_url(&url).await
}

/// Todo:
/// - handle chunks as we go to spawn new threads
/// - handle redirections (301 Moved Permanently). Can be triggered with package = Django (capital
///   D) for example
async fn fetch_url(url: &str) -> Result<()> {
    let uri = url.to_string().parse::<hyper::Uri>().unwrap();
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut resp = client.get(uri).await?;

    let resp_status = resp.status();
    if resp_status.as_u16() >= 400 {
        panic!("HTTP status {} for url {}", resp_status, url);
    }

    let mut response: String = "".to_owned();
    while let Some(next) = resp.data().await {
        let chunk = next?;
        let chunk_str = String::from_utf8((&chunk).to_vec())?;
        response.push_str(&chunk_str);
    }
    println!("{}", response);

    Ok(())
}
