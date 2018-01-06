extern crate html5ever;
extern crate url;

use std::env;
use std::io::stdout;
use std::io::Write;
use url::Url;

use fetch::UrlState;

mod parse;
mod fetch;
mod crawler;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let start_url_string = &args[1];

        let start_url = Url::parse(start_url_string).unwrap();
        let domain = start_url
            .domain()
            .expect("I can't find a domain in your URL");

        let mut success_count = 0;
        let mut fail_count = 0;

        for url_state in crawler::crawl(&domain, &start_url) {
            match url_state {
                UrlState::Accessible(_) => {
                    success_count += 1;
                }
                status => {
                    fail_count += 1;
                    println!("{}", status);
                }
            }

            print!("Succeeded: {} Failed: {}\r", success_count, fail_count);
            stdout().flush().unwrap();
        }
    } else {
        println!("Please provide a URL.");
    }
}
