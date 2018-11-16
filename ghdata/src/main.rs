extern crate github_rs;
extern crate serde_json;
extern crate mysql;
extern crate regex;
use github_rs::client::{Executor, Github};
use serde_json::Value;
use serde_json::to_string_pretty;
use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("../DETAILS").unwrap();
    let re_find = Regex::new(r"token:[ \t]*([0-9a-f]+)[\n]").unwrap();
    let token = &re_find.captures(&file).unwrap()[1];
    println!("{}", token);
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
    //let pool = mysql::Pool::new("mysql://et-vis:et-vis@localhost:3307/et-vis").unwrap();

}
