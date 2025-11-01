use std::collections::HashMap;
use anyhow::{Context, Result};
use reqwest::header::USER_AGENT;
use url::Url;

use crate::{models::{EventType, Response}, Username};

pub async fn github_user_activities(username: Username) -> Result<()> {
    let base_url = format!("https://api.github.com/users/{}/events", username.name);
    let parsed_url = Url::parse(&base_url)
        .context("failed to parse GitHub API URL")?;

    let client = reqwest::Client::new();
    let resp = client
        .get(parsed_url)
        .header(USER_AGENT, "user-activity-rs")
        .send()
        .await
        .context("network request to GitHub API failed")?;

    // Provide a friendly error when GitHub returns a non-success status.
    let resp = resp
        .error_for_status()
        .with_context(|| format!("GitHub API returned an error for user '{}'. Check the username or rate limits.", username.name))?;

    let response_body = resp.text().await.context("failed to read response body from GitHub")?;
    let events: Vec<Response> = serde_json::from_str(&response_body)
        .context("failed to parse JSON response from GitHub")?;

    print_activities(events).await?;
    Ok(())
}

pub async fn print_activities(body: Vec<Response>) -> Result<()> {
    let mut events_map = HashMap::new();
    let mut push_event_details: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    let mut create_event_details: HashMap<String, i32> = HashMap::new();
    let mut fork_event_details: HashMap<String, i32> = HashMap::new();
    let mut watch_event_details: HashMap<String, i32> = HashMap::new();
    let mut public_event_details: HashMap<String, i32> = HashMap::new();
    let mut issue_event_details: HashMap<String, i32> = HashMap::new();
    let mut issue_comment_event_details: HashMap<String, i32> = HashMap::new();
    let mut pull_request_details: HashMap<String, i32> = HashMap::new();
    let mut unknown_event_details: HashMap<String, i32> = HashMap::new();

    for event in body {
        *events_map.entry(event.event_type).or_insert(0) += 1;
        match event.event_type {
            EventType::PushEvent => {
                let (display_login, counter): (String, i32) = (event.actor.display_login, 0);
                let value = push_event_details.entry(event.repo.name).or_insert(vec![(display_login, counter)]);
                value[0].1 += 1;
            }
            EventType::CreateEvent => {
                *create_event_details.entry(event.repo.name).or_insert(0) += 1;
            }
            EventType::ForkEvent => {
                *fork_event_details.entry(event.repo.name).or_insert(0) += 1;
            }
            EventType::WatchEvent => {
                *watch_event_details.entry(event.repo.name).or_insert(0) += 1;
            }
            EventType::PublicEvent => {
                *public_event_details.entry(event.repo.name).or_insert(0) += 1;
            }
            EventType::IssueCommentEvent => {
                *issue_comment_event_details.entry(event.repo.name).or_insert(0) += 1;
            }
            EventType::IssuesEvent => {
                *issue_event_details.entry(event.repo.name).or_insert(0) += 1;
            }
            EventType::PullRequestEvent => {
                *pull_request_details.entry(event.repo.name).or_insert(0) += 1;
            }
            EventType::Unknown => {
                *unknown_event_details.entry(event.repo.name).or_insert(0) += 1;
            }
        }
    }

    for (event_type, _) in events_map {
        match event_type {
            EventType::PushEvent => {
                for (repo, commits) in &push_event_details {
                    let result = format!("{} Pushed {} commits to {}", commits[0].0, commits[0].1, repo);
                    println!("{}", result);
                }
            }
            EventType::CreateEvent => {
                for (repo, _) in &create_event_details {
                    let result = format!("Created new repo {}", repo);
                    println!("{}", result);
                }
            }
            EventType::ForkEvent => {
                for (repo, _) in &fork_event_details {
                    let result = format!("Forked the repo {}", repo);
                    println!("{}", result);
                }
            }
            EventType::WatchEvent => {
                for (repo, _) in &watch_event_details {
                    let result = format!("Starred the repo {}", repo);
                    println!("{}", result);
                }
            }
            EventType::PublicEvent => {
                for (repo, _) in &public_event_details {
                    let result = format!("Made this {} repo public", repo);
                    println!("{}", result);
                }
            }
            EventType::IssueCommentEvent => {
                for (repo, commit) in &issue_comment_event_details {
                    let result = format!("Commented {} messages in repo {}", commit, repo);
                    println!("{}", result);
                }
            }
            EventType::IssuesEvent => {
                for (repo, commit) in &issue_event_details {
                    let result = format!("Created {} new issues in repo {}", commit, repo);
                    println!("{}", result);
                }
            }
            EventType::PullRequestEvent => {
                for (repo, _) in &pull_request_details {
                    let result = format!("Created new pull request in repo {}", repo);
                    println!("{}", result);
                }
            }
            EventType::Unknown => {
                for (repo, _) in &unknown_event_details {
                    let result = format!("User did something in repo {}", repo);
                    println!("{}", result);
                }
            }
        }
    }

    Ok(())
}