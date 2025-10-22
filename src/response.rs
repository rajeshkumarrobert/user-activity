use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename="PascalCase")]
pub enum EventType{
    PushEvent,
    IssuesEvent,
    PullRequestEvent,
    IssueCommentEvent,
    #[serde(other)]
    Unknown
}

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

#[derive(Deserialize, Debug)]
pub struct Actor{
    id: i64,
    login: String,
    display_login: String,
    gravatar_id: Option<String>,
    url: String,
    avatar_url: String
}

#[derive(Deserialize, Debug)]
pub struct Payload{
    repository_id: i64,
    push_id: i64,
    r#ref: String,
    head: String,
    before: String
}

#[derive(Deserialize, Debug)]
pub struct Repo{
    pub id: i32,
    pub name: String,
    pub url: String
}