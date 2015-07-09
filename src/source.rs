use std::io::{Read};
use std::fmt;
use hyper::{Url};
use hyper::client::{Client, IntoUrl};
use hyper::header::{UserAgent};
use url::ParseError as UrlError;
use rustc_serialize::json::Json;

const IS_GD_API_URL: &'static str = "http://is.gd/forward.php";
const GIST_API_URL: &'static str = "https://api.github.com/gists";

#[derive(Debug, Clone)]
pub enum Source {
    IsGd(String),
    Gist(String),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let me = self.clone();
        match me {
            Source::IsGd(url) => IsGd(url).fmt(f),
            Source::Gist(id) => Gist(id).fmt(f),
        }
    }
}

impl IntoUrl for Source {
    fn into_url(self) -> Result<Url, UrlError> {
        match self {
            Source::IsGd(url) => IsGd(url).into_url(),
            Source::Gist(id) => Gist(id).into_url(),
        }
    }
}

#[derive(Debug, Clone)]
struct IsGd(String);
#[derive(Debug, Clone)]
struct Gist(String);

fn readall<T: IntoUrl>(obj: T, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let client = Client::new();
    let url = obj.into_url().unwrap();

    let req = client.get(url).header(UserAgent("cargo-cp".into()));


    match req.send() {
        Ok(mut resp) => {
            let mut body: String = "".into();
            resp.read_to_string(&mut body).unwrap();
            let obj = match Json::from_str(&body).unwrap() {
                Json::Object(obj) => obj,
                _ => panic!("at the disco"),
            };
            let maybe_file = obj.get("files").unwrap();
            let files = match maybe_file.clone() {
                Json::Object(obj) => obj,
                _ => panic!("at the disco"),
            };
            let maybe_thefile = files.get("playground.rs").unwrap();
            let thefile = match maybe_thefile.clone() {
                Json::Object(obj) => obj,
                _ => panic!("at the disco"),
            };
            let maybe_thecontent = thefile.get("content").unwrap();
            let content = match maybe_thecontent.clone() {
                Json::String(s) => s,
                _ => panic!("at the disco"),
            };
            write!(f, "{:?}", content)
        },
        Err(_) => panic!("at the disco"),
    }
}

impl fmt::Display for IsGd {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let me = self.clone();
        let url = match me.into_url() {
            Ok(url) => url,
            Err(e) => panic!("Couldn't complete into_url(), error was {:?}", e),
        };
        let code = url.query_pairs().unwrap().into_iter().find(|&(ref key, _)| { key == "code" });
        write!(f, "{:?}", code.unwrap().1)
    }
}

impl fmt::Display for Gist {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let me = self.clone();
        readall(me, f)
    }
}

/// TODO: get rid of `.unwrap()` calls and panic!s
///
/// the is.gd api has a "simple" mode that just returns the long url as the only part of the body
impl IntoUrl for IsGd {
    fn into_url(self) -> Result<Url, UrlError> {
        let client = Client::new();

        let mut baseurl = try!(IS_GD_API_URL.into_url());
        baseurl.set_query_from_pairs(vec![("format", "simple"), ("shorturl", &self.0)].into_iter());
        let req = client.get(baseurl);

        match req.send() {
            Ok(mut resp) => {
                let mut longurl: String = "".into();
                if let Err(e) = resp.read_to_string(&mut longurl) {
                    panic!("Couldn't read_to_string, error was {:?}", e);
                }
                Ok(try!(longurl.into_url()))
            },
            Err(_) => panic!("AHHH"),
        }
    }
}

impl IntoUrl for Gist {
    fn into_url(self) -> Result<Url, UrlError> {
        let url = vec![GIST_API_URL, &self.0].connect("/");
        url.into_url()
    }
}
