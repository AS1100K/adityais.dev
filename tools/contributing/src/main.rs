use tool_contributing::GitHubClient;

const USER: &str = "AS1100K";

#[tokio::main]
async fn main() {
    let token = std::env::var("TOOLS_CONTRIBUTING_TOKEN")
        .expect("Please provide the `TOOLS_CONTRIBUTING_TOKEN` env variable");

    let mut client = GitHubClient::new(token, String::from(USER));
    client.auto().await;
    client.save();
}
