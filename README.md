# user_activity — GitHub user activity CLI
![Rust CI](https://github.com/rajeshkumarrobert/user-activity/actions/workflows/ci.yml/badge.svg)

Project idea got from [roadmap.sh](https://roadmap.sh/projects/github-user-activity)

A small, focused Rust CLI that fetches and prints recent public events for a GitHub user.

- Entry point: [src/main.rs](src/main.rs) — sees the CLI `Username` parser, [`Username`](src/main.rs).
- Fetch & format logic: [src/handler.rs](src/handler.rs) — main function [`crate::handler::github_user_activities`](src/handler.rs).
- JSON models: [src/models.rs](src/models.rs) — types like [`models::Response`](src/models.rs) and [`models::EventType`](src/models.rs).
- Build config: [Cargo.toml](Cargo.toml)

Why this exists
- Quick way to inspect what a user has been doing on GitHub (pushes, forks, issues, etc.).
- Small codebase to learn async HTTP requests with `reqwest` and simple serde-based parsing.

Usage

```sh
# build and run for user "alice"
cargo run -- alice

The CLI expects a single positional argument (the GitHub username).

What it prints

Human-friendly lines describing detected events, e.g.:
"alice Pushed 3 commits to alice/repo"
"Forked the repo alice/repo"
"Starred the repo alice/repo"
Enjoy exploring GitHub activity! ``````