extern crate github_rs;
extern crate serde_json;
extern crate mysql;
use github_rs::client::{Executor, Github};
use serde_json::Value;
use serde_json::to_string_pretty;


fn main() {
    let client = Github::new("af374f41456a915d346f97cfdd42cf1d30df0b58").unwrap();
    let me = client.get()
        .repos()
        .owner("rust-lang")
        .repo("rust")
        .contributors()
        .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", to_string_pretty(&json).unwrap());
            }
        },
        Err(e) => println!("{}", e)
    }
}
