use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct RepoName(String, String);

impl RepoName {
    pub fn new<O: AsRef<str>, R: AsRef<str>>(owner: O, repo: R) -> Self {
        Self(String::from(owner.as_ref()), String::from(repo.as_ref()))
    }
}

impl Display for RepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl Serialize for RepoName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize)]
pub struct RepoInfo {
    pub stars: usize,
    pub avatar_url: String,
}

pub type RepoInfoMap = HashMap<RepoName, RepoInfo>;

#[derive(Debug)]
pub struct GitHubStargazerCount<'a> {
    client: reqwest::Client,
    repos: &'a HashSet<RepoName>,
    token: String,
    pub repo_info: RepoInfoMap,
}

impl<'a> GitHubStargazerCount<'a> {
    const MAX_RETRIES: u8 = 3;

    const GET_REPO: &'a str = "https://api.github.com/repos";

    const FILE_NAME: &'a str = "repo-info.json";

    const ACCEPT_HEADER: &'a str = "application/vnd.github+json";
    const GITHUB_API_VERSION_HEADER: &'a str = "2022-11-28";
    const USER_AGENT_HEADER: &'a str = "adityais.dev-tools";

    pub fn new(token: String, repos: &'a HashSet<RepoName>) -> Self {
        let client = reqwest::Client::new();
        let star_count = HashMap::new();

        Self {
            client,
            repos,
            token,
            repo_info: star_count,
        }
    }

    pub async fn auto(&mut self) {
        #[derive(Deserialize)]
        struct GetRepoRes {
            stargazers_count: usize,
            owner: Owner,
        }

        #[derive(Deserialize)]
        struct Owner {
            avatar_url: String,
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

                let repo_info = RepoInfo {
                    stars: res.stargazers_count,
                    avatar_url: res.owner.avatar_url,
                };
                self.repo_info.insert(repo.to_owned(), repo_info);

                break;
            }
        }
    }

    pub fn save(self) {
        let json =
            serde_json::to_string(&self.repo_info).expect("Failed to convert start count in json");
        fs::write(Self::FILE_NAME, json).expect("Failed to write to the file");
    }
}
