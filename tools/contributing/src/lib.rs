use serde::{
    de::{self, Visitor},
    Deserialize,
};

#[derive(Debug, Deserialize, PartialEq)]
pub struct SearchRes {
    pub total_count: u32,
    pub items: Vec<PullRequest>,
}

#[derive(Debug, PartialEq)]
pub struct PullRequest {
    pub html_url: String,
    pub title: String,
    pub number: usize,
    pub owner_repo: (String, String),
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub state: PullRequestState,
}

#[derive(Debug, PartialEq)]
pub enum PullRequestState {
    Open,
    Draft,
    Closed,
    Merged,
}

#[derive(Debug, Deserialize, PartialEq)]
struct InnerPullRequest {
    merged_at: Option<String>,
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
            UpdatedAt,
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
                let mut html_url = None;
                let mut title = None;
                let mut number = None;
                let mut updated_at = None;
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
                        Field::UpdatedAt => {
                            if updated_at.is_some() {
                                return Err(de::Error::duplicate_field("updated_at"));
                            }

                            updated_at = Some(map.next_value()?)
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
                let updated_at =
                    updated_at.ok_or_else(|| de::Error::missing_field("updated_at"))?;

                let draft: bool = draft.ok_or_else(|| de::Error::missing_field("draaft"))?;
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
                    updated_at,
                    state: our_state,
                })
            }
        }

        deserializer.deserialize_map(PullRequestVisitor)
    }
}
