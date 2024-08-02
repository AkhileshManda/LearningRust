use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let user_name = "abc".to_string();
    let password: Option<String> = Some(String::from("abc"));
    let response = client
        .get("http://httpbin.org/")
        .basic_auth(user_name, password)
        .send().await?;

    // let resStatus = response.();

    println!("{:?}", response);

    Ok(())
}
