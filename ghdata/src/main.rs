extern crate github_rs;
extern crate serde_json;
extern crate mysql;
extern crate regex;
use github_rs::client::{Executor, Github};
use serde_json::Value;
use serde_json::to_string;
use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("../DETAILS").unwrap();

    let re_find_token = Regex::new(r"token:[ \t]*([0-9a-f]+)[\n]").unwrap();
    let token = &re_find_token.captures(&file).unwrap()[1];

    let client = Github::new(token).unwrap();
    let initial = client.get()
        .repos()
        .owner("rust-lang")
        .repo("rust")
        .contributors()
        .execute::<Value>();
    let mut names :Vec<String> = Vec::new();
    match initial {
        Ok((_headers, _status, json)) => {
            //println!("{}", headers);
            //println!("{}", status);
            if let Some(json) = json{
                //println!("{}", to_string_pretty(&json).unwrap());
                //let result = to_string(&json).unwrap();
                for contrib in 0..30 {
                    names.push(to_string(&json[contrib]["login"]).unwrap().replace("\"",""));
                }
                for name in names {
                    println!("{}",name);
                }
            }
        },
        Err(e) => println!("{}", e)
    }

    /*
    let re_find_msuser = Regex::new(r"msuser:[ \t]*(?P<user>.+)[\n]").unwrap();
    let re_find_mspass = Regex::new(r"mspass:[ \t]*(?P<pass>.+)[\n]").unwrap();
    let msuser = &re_find_msuser.captures(&file).unwrap()[1];
    let mspass = &re_find_mspass.captures(&file).unwrap()[1];
    let mut connection = String::from("mysql://");
    connection.push_str(msuser);
    connection.push_str(":");
    connection.push_str(mspass);
    connection.push_str("@localhost:3306/et-vis");
    let pool = mysql::Pool::new(&connection).unwrap();
    pool.prep_exec("SHOW TABLES",()).map(
        |result|{
          for row in result {
              let val : String = mysql::from_row(row.unwrap());
              println!("{}",val);
          }
        }).unwrap();
    */
}
