extern crate github_rs;
extern crate serde_json;
#[macro_use]
extern crate mysql;
extern crate regex;

use github_rs::client::{Executor, Github};
use serde_json::Value;
use serde_json::to_string;
//use serde_json::to_string_pretty;
use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("../DETAILS").unwrap();

    let re_find_token = Regex::new(r"token:[ \t]*([0-9a-f]+)[\n]").unwrap();
    let token = &re_find_token.captures(&file).unwrap()[1];

    let client = Github::new(token).unwrap();
    let userrepos = client.get()
        .users()
        .username("rust-lang")
        .repos()
        .execute::<Value>();
    let mut values = Vec::new();
    match userrepos {
        Ok((_,_,json)) => {
            if let Some(json) = json {
                for repo in json.as_array().unwrap() {
                    values.push((to_string(&repo["name"]).unwrap().replace("\"",""),
                                 to_string(&repo["watchers"]).unwrap().parse::<u32>().unwrap()));
                    println!("{} - {}", to_string(&repo["full_name"]).unwrap().replace("\"",""),
                             to_string(&repo["watchers"]).unwrap());
                }
            }
        }
        Err(e) => println!("{}", e)
    }
    println!("{:?}",values);
    /*
    let initial = client.get()
        .repos()
        .owner("rust-lang")
        .repo("rust")
        .contributors()
        .execute::<Value>();
    let mut names: Vec<String> = Vec::new();
    match initial {
        Ok((_headers, _status, json)) => {
            //println!("{}", headers);
            //println!("{}", status);
            if let Some(json) = json {
                //println!("{}", to_string_pretty(&json).unwrap());
                //let result = to_string(&json).unwrap();
                for contrib in 0..30 {
                    names.push(to_string(&json[contrib]["login"]).unwrap().replace("\"", ""));
                }
                for name in names {
                    println!("{}", name);
                    let repos = client.get().users().username(&name).repos().execute::<Value>();
                    match repos {
                        Ok((_, _, repo_json)) => {
                            if let Some(repo_json) = repo_json {
                                let repo_vec = repo_json.as_array().unwrap();
                                for owned_repo in repo_vec {
                                    println!("{}", to_string_pretty(&owned_repo["full_name"]).unwrap());
                                }
                            }
                        }
                        Err(e) => println!("{}", e)
                    }
                }
            }
        }
        Err(e) => println!("{}", e)
    }
    */

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
    for mut statement in pool.prepare(r"INSERT INTO et_vis_graph
        (owner, repo, viewers) VALUES (:owner, :repo, :viewers)").into_iter() {
        for value in &values {
            statement.execute( params!{
                    "owner" => "rust_lang",
                    "repo" => &value.0,
                    "viewers" => value.1
                }
            ).unwrap();
        }
    }
}
