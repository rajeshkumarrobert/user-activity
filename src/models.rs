use serde::Deserialize;

#[derive(Deserialize,Clone, Copy, PartialEq, Debug, Eq, Hash)]
#[serde(rename="PascalCase")]
pub enum EventType{
    PushEvent,
    IssuesEvent,
    PullRequestEvent,
    IssueCommentEvent,
    WatchEvent,
    ForkEvent,
    CreateEvent,
    PublicEvent,
    #[serde(other)]
    Unknown
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Response{
    pub id: String,
    pub actor: Actor,
    #[serde(rename="type")]
    pub event_type: EventType,
    pub repo: Repo,
    pub public: bool,
    pub payload: serde_json::Value,
    pub created_at: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Actor{
    pub id: i64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub avatar_url: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Payload{
    repository_id: i64,
    push_id: i64,
    r#ref: String,
    head: String,
    before: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Repo{
    pub id: i32,
    pub name: String,
    pub url: String
}