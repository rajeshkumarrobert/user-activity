use clap::Parser;
use crate::handler::github_user_activities;

mod models;
mod handler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Username {
    name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let user_name = Username::parse();

    // Run the handler and present user-friendly errors with context.
    if let Err(e) = github_user_activities(user_name).await {
        eprintln!("Error: {:#}", e);

        // A small heuristic: if the error mentions rate limit, add a hint.
        if format!("{}", e).to_lowercase().contains("rate limit") {
            eprintln!("Hint: unauthenticated requests to the GitHub API are rate-limited. Consider using a token or try again later.");
        }

        std::process::exit(1);
    }

    Ok(())
}

