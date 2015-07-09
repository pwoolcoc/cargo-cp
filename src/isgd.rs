use hyper::{Url};
use hyper::client::{IntoUrl};
use url::ParseError as UrlError;

enum PlaygroundSource {
    IsGdUrl(String),
    Gist(Id),
}

impl IntoUrl for PlaygroundSource {
    fn into_url(self) -> Result<Url, UrlError> {
    }
}

fn get_isgd_url(url: &str) -> Result<String, Box<Error>> {
    let client = Client::new();
    let url = format!("http://is.gd/forward.php?shorturl={:?}&format=simple", url);
    let req = client.get(&url);
    match req.send() {
        Ok(mut resp) => {
            let mut long_url: String = "".into();
            resp.read_to_string(&mut long_url).unwrap();
            Ok(long_url)
        },
        Err(_) => panic!("Error getting long url from is.gd"),
    }
}

