use std::{collections::HashMap, io::{Error, ErrorKind}};
use reqwest::header::USER_AGENT;
use url::Url;

use crate::{models::{EventType, Response}, Username};

pub async fn github_user_activities(username:Username) ->Result<(),Error>{
    let base_url = format!("https://api.github.com/users/{}/events",username.name);
    let parsed_url = Url::parse(&base_url)
        .map_err(|e| Error::new(ErrorKind::InvalidInput,e))?;
    let client = reqwest::Client::new();
    let response = client
                                .get(parsed_url)
                                .header(USER_AGENT, "Rust-CLI-App")
                                .send()
                                .await
                                .map_err(|e|Error::new(ErrorKind::NotFound, e))?;
        
    let response_body = response.text().await.map_err(|e|Error::new(ErrorKind::InvalidData, e))?;
    let events:Vec<Response> = serde_json::from_str(&response_body)?;
    print_activites(events).await?;
    
    Ok(())
}

pub async fn print_activites(body:Vec<Response>) -> Result<(),Error>{
    let mut events_map = HashMap::new();
    let mut push_event_details = HashMap::new();
    let mut create_event_details = HashMap::new();
    let mut fork_event_details = HashMap::new();
    let mut watch_event_details = HashMap::new();
    let mut public_event_details = HashMap::new();
    let mut issue_event_details = HashMap::new();
    let mut issue_comment_event_details = HashMap::new();
    let mut pull_request_details = HashMap::new();
    let mut unknown_event_details = HashMap::new();
    for event in body {
        *events_map.entry(event.event_type).or_insert(0)+=1;
        match event.event_type {
            EventType::PushEvent =>{
            let (display_login,counter):(String,i32) = (event.actor.display_login, 0);
            let value = push_event_details.entry(event.repo.name).or_insert(vec![(display_login,counter)]);
            value[0].1+=1;
        },
        EventType::CreateEvent =>{
             *create_event_details.entry(event.repo.name).or_insert(0)+=1;
        },
        EventType::ForkEvent =>{
            *fork_event_details.entry(event.repo.name).or_insert(0)+=1;
        },
        EventType::WatchEvent =>{
            *watch_event_details.entry(event.repo.name).or_insert(0)+=1;
        },
        EventType::PublicEvent =>{
            *public_event_details.entry(event.repo.name).or_insert(0)+=1;
        },
        EventType::IssueCommentEvent =>{
            *issue_comment_event_details.entry(event.repo.name).or_insert(0)+=1;
        },
        EventType::IssuesEvent =>{
           *issue_event_details.entry(event.repo.name).or_insert(0)+=1;
        },
        EventType::PullRequestEvent =>{
            *pull_request_details.entry(event.repo.name).or_insert(0)+=1;
        },
        EventType::Unknown =>{
            *unknown_event_details.entry(event.repo.name).or_insert(0)+=1;
        }
        }
    }
    for (event_type,_) in events_map{
    match event_type {
        EventType::PushEvent =>{
            for (repo,commits) in &push_event_details{
                let result = format!("{} Pushed {} commits to {}",commits[0].0, commits[0].1, repo);
                eprintln!("{}",result);
            }
        },
        EventType::CreateEvent =>{
            for (repo,_) in &create_event_details{
                let result = format!("Created new repo {}", repo);
                eprintln!("{}",result);
            }
        },
        EventType::ForkEvent =>{
            for (repo,_) in &fork_event_details{
                let result = format!("Forked the repo {}", repo);
                eprintln!("{}",result);
            }
        },
        EventType::WatchEvent =>{
            for (repo,_) in &watch_event_details{
                let result = format!("Starred the repo {}", repo);
                eprintln!("{}",result);
            }
        },
        EventType::PublicEvent =>{
            for (repo,_) in &public_event_details{
                let result = format!("Made this {} repo as public", repo);
                eprintln!("{}",result);
            }
        },
        EventType::IssueCommentEvent =>{
            for (repo,commit) in &issue_comment_event_details{
                let result = format!("Commented {} messgaes in repo {}", commit, repo);
                eprintln!("{}",result);
            }
        },
        EventType::IssuesEvent =>{
            for (repo,commit) in &issue_event_details{
                let result = format!("Created new {} issues in repo {},", commit, repo);
                eprintln!("{}",result);
            }
        },
        EventType::PullRequestEvent =>{
            for (repo,_) in &pull_request_details{
                let result = format!("Created new pullrequest in repo {}", repo);
                eprintln!("{}",result);
            }
        },
        EventType::Unknown =>{
            for (repo,_) in &unknown_event_details{
                let result = format!("User did somthing in repo {}", repo);
                eprintln!("{}",result);
            }
        }
    }
    
    }
   Ok(())
}