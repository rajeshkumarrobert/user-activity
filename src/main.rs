use std::{collections::HashSet, io::{Error,ErrorKind}};
use clap::Parser;
use reqwest::header::USER_AGENT;
use url::Url;

use crate::response::{EventType, Response};

mod response;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Username{
    name: String
}
#[tokio::main]
async fn main()->Result<(),Error> {
    // println!("Hello, world!");
    // println!("github-activity");

    // let mut user_name = String::new();
    //  io::stdin().read_line(&mut user_name).expect("failed to read line");

    //  println!("The github username is :{user_name}");

    let user_name = Username::parse();
    println!("Hello, {}!", user_name.name);
    let url = format!("https://api.github.com/users/{}/events",user_name.name);
    let parsed_url = Url::parse(&url)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    let client = reqwest::Client::new();
    let response = client
    .get(parsed_url)
    .header(USER_AGENT, "Testing-reqwest")
    .send()
    .await
    .map_err(|e|Error::new(ErrorKind::NotFound, e))?;
    
    let body = response.text().await.map_err(|e|Error::new(ErrorKind::InvalidData, e))?;

    let value:Vec<Response> = serde_json::from_str(&body)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    let push_event_value = value.iter().filter(|x| x.event_type == EventType::PushEvent).collect::<Vec<_>>();
    let issues_event_value = value.iter().filter(|x| x.event_type == EventType::IssuesEvent).collect::<Vec<_>>();
    let pullrequest_event_value = value.iter().filter(|x| x.event_type == EventType::PullRequestEvent).collect::<Vec<_>>();
    let mut push_repo_list = HashSet::new();
    for x in &push_event_value{
        push_repo_list.insert(x.repo.name.clone());
    }
    //let pushed_event_count = value.iter().filter(|x| x.event_type == EventType::PushEvent).count();
    eprint!("Output:\n");
    eprintln!("Pushed {:?} in the following repo's {:?}",push_event_value.len(), push_repo_list);
    eprintln!("Created {:?} issues in the last month",issues_event_value.len());
    eprintln!("Created {:?} pull request in the last month",pullrequest_event_value.len());
    Ok(())
}

