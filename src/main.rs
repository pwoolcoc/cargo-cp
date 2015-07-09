extern crate rustc_serialize;
extern crate url;
extern crate hyper;

mod source;

use std::env;

use hyper::client::IntoUrl;
use source::Source;

#[cfg(not(test))]
fn main() {
    // first 2 args are `cargo-cp`, `cp`, so skip them
    let args = env::args().skip(2).collect::<Vec<_>>();
    let url = match args.first() {
        Some(url) => url,
        None => panic!("Need to pass a playground URL or gist id"),
    };

    let url = url.to_owned();
    if let Ok(_) = url.clone().into_url() {
        println!("{}", Source::IsGd(url));
    } else {
        println!("{}", Source::Gist(url));
    }
    // let source = Source::IsGd("http://is.gd/YWqfuw".into());
    // let source = Source::Gist("700394a6fba8e1a339f4".into());
}

#[cfg(test)] mod tests {
    #[test]
    fn test_me() {
        assert!(false);
    }
}
