// use error_chain::error_chain;
use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::{Deserialize, Serialize};

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    let client = reqwest::Client::new();

    let res = client
        .get(&api_url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let users: Vec<User> = res.json().await?;
    println!("Users are  : {:?}", users);
    Ok(())
}
