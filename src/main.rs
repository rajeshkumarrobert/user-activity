use std::io::Error;
use clap::Parser;
use crate::handler::github_user_activities;

mod models;
mod handler;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Username{
    name: String
}
#[tokio::main]
async fn main()->Result<(),Error> {

    let user_name = Username::parse();
    println!("Hello, {}!", user_name.name);
    github_user_activities(user_name).await?;
    Ok(())
}

