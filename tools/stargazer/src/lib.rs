use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct RepoInfo(String, String);

impl RepoInfo {
    pub fn new<O: AsRef<str>, R: AsRef<str>>(owner: O, repo: R) -> Self {
        Self(String::from(owner.as_ref()), String::from(repo.as_ref()))
    }
}

impl Display for RepoInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl Serialize for RepoInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type RepoStarCount = HashMap<RepoInfo, usize>;

#[derive(Debug)]
pub struct GitHubStargazerCount<'a> {
    client: reqwest::Client,
    repos: &'a HashSet<RepoInfo>,
    token: String,
    pub star_count: RepoStarCount,
}

impl<'a> GitHubStargazerCount<'a> {
    const MAX_RETRIES: u8 = 3;

    const GET_REPO: &'a str = "https://api.github.com/repos";

    const FILE_NAME: &'a str = "stargazer-count.json";

    const ACCEPT_HEADER: &'a str = "application/vnd.github+json";
    const GITHUB_API_VERSION_HEADER: &'a str = "2022-11-28";
    const USER_AGENT_HEADER: &'a str = "adityais.dev-tools";

    pub fn new(token: String, repos: &'a HashSet<RepoInfo>) -> Self {
        let client = reqwest::Client::new();
        let star_count = HashMap::new();

        Self {
            client,
            repos,
            token,
            star_count,
        }
    }

    pub async fn auto(&mut self) {
        #[derive(Deserialize)]
        struct GetRepoRes {
            stargazers_count: usize,
        }

        for repo in self.repos {
            let mut current_retry_number: u8 = 0;

            while current_retry_number <= Self::MAX_RETRIES {
                let url = format!("{}/{}/{}", Self::GET_REPO, repo.0, repo.1);

                let res = self
                    .client
                    .get(&url)
                    .bearer_auth(&self.token)
                    .header("Accept", Self::ACCEPT_HEADER)
                    .header("X-GitHub-Api-Version", Self::GITHUB_API_VERSION_HEADER)
                    .header("User-Agent", Self::USER_AGENT_HEADER)
                    .send()
                    .await
                    .expect("Failed to construct request");

                let res = match res.status().as_u16() {
                    403 | 301 => {
                        eprintln!(
                            "Got Status Code: {}. Response: {:?}",
                            res.status(),
                            res.bytes()
                                .await
                                .expect("Failed to convert error response into bytes")
                        );

                        // Move to next repo
                        break;
                    }
                    404 => {
                        // Move to next repo
                        eprintln!("Got 404 for {}", url);
                        break;
                    }
                    200 | 201 => {
                        let bytes = res
                            .bytes()
                            .await
                            .expect("Failed to convert response to bytes");

                        let res: GetRepoRes = serde_json::from_slice(&bytes)
                            .expect("Failed to deserialize the body response");
                        res
                    }
                    _ => {
                        current_retry_number += 1;

                        eprintln!(
                            "Failed to fetch response from the search API. Retrying... Error: {:?}",
                            res.bytes()
                                .await
                                .expect("Failed to convert error response into bytes")
                        );
                        continue;
                    }
                };

                self.star_count
                    .insert(repo.to_owned(), res.stargazers_count);

                break;
            }
        }
    }

    pub fn save(self) {
        let json =
            serde_json::to_string(&self.star_count).expect("Failed to convert start count in json");
        fs::write(Self::FILE_NAME, json).expect("Failed to write to the file");
    }
}
