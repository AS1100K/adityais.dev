use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};
use tool_repo_info::RepoName;

#[derive(Debug, Deserialize, PartialEq)]
pub struct SearchRes {
    pub total_count: u32,
    pub items: Vec<PullRequest>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct PullRequest {
    pub html_url: String,
    pub title: String,
    pub number: usize,
    pub owner_repo: (String, String),
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub state: PullRequestState,
    pub owner_avatar_url: String,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum PullRequestState {
    Open,
    Draft,
    Closed,
    Merged,
}

impl<'de> Deserialize<'de> for PullRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Field {
            HtmlUrl,
            Title,
            Number,
            CreatedAt,
            #[serde(rename = "user")]
            OwnerAvatarURL,
            State,
            PullRequest,
            Draft,
            #[serde(other)]
            Ignore,
        }

        struct PullRequestVisitor;

        impl<'de> Visitor<'de> for PullRequestVisitor {
            type Value = PullRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "struct PullRequest")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                #[derive(Deserialize, PartialEq)]
                struct InnerPullRequest {
                    merged_at: Option<String>,
                }

                #[derive(Deserialize)]
                struct InnerUser {
                    avatar_url: String,
                }

                let mut html_url = None;
                let mut title = None;
                let mut number = None;
                let mut created_at = None;
                let mut avatar_url = None;
                let mut draft = None;
                let mut inner_pull_request = None;
                let mut state = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::HtmlUrl => {
                            if html_url.is_some() {
                                return Err(de::Error::duplicate_field("html_url"));
                            }

                            html_url = Some(map.next_value()?)
                        }
                        Field::Title => {
                            if title.is_some() {
                                return Err(de::Error::duplicate_field("title"));
                            }

                            title = Some(map.next_value()?)
                        }
                        Field::Number => {
                            if number.is_some() {
                                return Err(de::Error::duplicate_field("number"));
                            }

                            number = Some(map.next_value()?)
                        }
                        Field::CreatedAt => {
                            if created_at.is_some() {
                                return Err(de::Error::duplicate_field("created_at"));
                            }

                            created_at = Some(map.next_value()?)
                        }
                        Field::OwnerAvatarURL => {
                            if avatar_url.is_some() {
                                return Err(de::Error::duplicate_field("user"));
                            }

                            avatar_url = Some(map.next_value()?)
                        }
                        Field::Draft => {
                            if draft.is_some() {
                                return Err(de::Error::duplicate_field("draft"));
                            }

                            draft = Some(map.next_value()?)
                        }
                        Field::State => {
                            if state.is_some() {
                                return Err(de::Error::duplicate_field("state"));
                            }

                            state = Some(map.next_value()?)
                        }
                        Field::PullRequest => {
                            if inner_pull_request.is_some() {
                                return Err(de::Error::duplicate_field("pull_request"));
                            }

                            inner_pull_request = Some(map.next_value()?)
                        }
                        Field::Ignore => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let html_url: String =
                    html_url.ok_or_else(|| de::Error::missing_field("html_url"))?;

                let url_parts: Vec<&str> = html_url.split('/').collect();
                let owner_repo = if url_parts.len() > 4 {
                    (url_parts[3].to_string(), url_parts[4].to_string())
                } else {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(&html_url),
                        &"expected a valid GitHub URL with owner and repo, e.g. \"https://github.com/AS1100K/cargo-wiki/pull/2\"",
                    ));
                };

                let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
                let number = number.ok_or_else(|| de::Error::missing_field("number"))?;
                let created_at =
                    created_at.ok_or_else(|| de::Error::missing_field("created_at"))?;

                let avatar_url: InnerUser =
                    avatar_url.ok_or_else(|| de::Error::missing_field("user"))?;
                let avatar_url = avatar_url.avatar_url;

                let draft: bool = draft.ok_or_else(|| de::Error::missing_field("draft"))?;
                let state: String = state.ok_or_else(|| de::Error::missing_field("state"))?;
                let inner_pull_request: InnerPullRequest =
                    inner_pull_request.ok_or_else(|| de::Error::missing_field("pull_request"))?;

                let our_state = if draft {
                    PullRequestState::Draft
                } else {
                    match state.as_str() {
                        "open" => PullRequestState::Open,
                        "closed" => {
                            if inner_pull_request.merged_at.is_some() {
                                PullRequestState::Merged
                            } else {
                                PullRequestState::Closed
                            }
                        }
                        _ => PullRequestState::Open,
                    }
                };

                Ok(PullRequest {
                    html_url,
                    owner_repo,
                    title,
                    number,
                    created_at,
                    owner_avatar_url: avatar_url,
                    state: our_state,
                })
            }
        }

        deserializer.deserialize_map(PullRequestVisitor)
    }
}

pub struct GitHubClient {
    token: String,
    user: String,
    client: reqwest::Client,
    prs: Vec<PullRequest>,
    repo_info: HashSet<RepoName>,
    star_count: tool_repo_info::RepoInfoMap,
}

impl GitHubClient {
    const MAX_RETRIES: u8 = 3;

    const SEARCH_ISSUES: &str = "https://api.github.com/search/issues";

    const PR_FILE_NAME: &str = "prs.json";
    const REPO_INFO_FILE_NAME: &str = "repo-info.json";

    const ACCEPT_HEADER: &str = "application/vnd.github+json";
    const GITHUB_API_VERSION_HEADER: &str = "2022-11-28";
    const USER_AGENT_HEADER: &str = "adityais.dev-tools";

    pub fn new(token: String, user: String) -> Self {
        Self {
            token,
            user,
            client: reqwest::Client::new(),
            prs: Vec::new(),
            repo_info: HashSet::new(),
            star_count: HashMap::new(),
        }
    }

    pub async fn auto(&mut self) {
        let mut current_retry_number: u8 = 0;

        let url = format!("{}?q=type:pr+author:{}", Self::SEARCH_ISSUES, self.user);
        let mut current_page = 1;
        let mut total_page = 1;

        while current_retry_number <= Self::MAX_RETRIES && current_page <= total_page {
            let current_page_string = current_page.to_string();
            let params = [
                ("sort", "updated"),
                ("per_page", "50"),
                ("page", current_page_string.as_str()),
            ];

            let res = self
                .client
                .get(&url)
                .query(&params)
                .bearer_auth(&self.token)
                .header("Accept", Self::ACCEPT_HEADER)
                .header("X-GitHub-Api-Version", Self::GITHUB_API_VERSION_HEADER)
                .header("User-Agent", Self::USER_AGENT_HEADER)
                .send()
                .await
                .expect("Failed to construct request");

            let res = match res.status().as_u16() {
                403 | 422 => {
                    panic!(
                        "Got Status Code: {}. Response: {:?}",
                        res.status(),
                        res.bytes()
                            .await
                            .expect("Failed to convert error response into bytes")
                    )
                }
                200 | 201 => {
                    let bytes = res
                        .bytes()
                        .await
                        .expect("Failed to convert response to bytes");

                    let res: SearchRes = serde_json::from_slice(&bytes)
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

            // Reset retry counter
            current_retry_number = 0;
            current_page += 1;

            // Calculate Total Pages
            total_page = (res.total_count as f64 / 50.0).ceil() as u32;

            // Process Pull Requests
            for pr in res.items {
                if PullRequestState::Closed == pr.state {
                    continue;
                }

                self.prs.push(pr.clone());
                let repo_info = RepoName::new(pr.owner_repo.0, pr.owner_repo.1);
                self.repo_info.insert(repo_info);
            }
        }

        // Process stars
        let mut star_count =
            tool_repo_info::GitHubStargazerCount::new(self.token.clone(), &self.repo_info);
        star_count.auto().await;

        self.star_count = star_count.repo_info;
    }

    pub fn save(self) {
        let json = serde_json::to_string(&self.prs).expect("Failed to convert the PRs in json");
        fs::write(Self::PR_FILE_NAME, json).expect("Failed to write to the file");

        let json =
            serde_json::to_string(&self.star_count).expect("Failed to convert the stars in json");
        fs::write(Self::REPO_INFO_FILE_NAME, json).expect("Failed to write to the file");
    }
}
