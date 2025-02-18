#![recursion_limit = "256"]
use std::str::FromStr;
use tool_contributing::*;

#[test]
fn test_contributing() {
    let json = serde_json::json!({
        "url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2",
        "repository_url": "https://api.github.com/repos/AS1100K/cargo-wiki",
        "labels_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/labels{/name}",
        "comments_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/comments",
        "events_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/events",
        "html_url": "https://github.com/AS1100K/cargo-wiki/pull/2",
        "id": 2855523382_usize,
        "node_id": "PR_kwDONhMnk86LV35t",
        "number": 2,
        "title": "Add Links for types",
        "user": {
            "login": "AS1100K",
            "id": 117935160,
            "node_id": "U_kgDOBweMOA",
            "avatar_url": "https://avatars.githubusercontent.com/u/117935160?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/AS1100K",
            "html_url": "https://github.com/AS1100K",
            "followers_url": "https://api.github.com/users/AS1100K/followers",
            "following_url": "https://api.github.com/users/AS1100K/following{/other_user}",
            "gists_url": "https://api.github.com/users/AS1100K/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/AS1100K/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/AS1100K/subscriptions",
            "organizations_url": "https://api.github.com/users/AS1100K/orgs",
            "repos_url": "https://api.github.com/users/AS1100K/repos",
            "events_url": "https://api.github.com/users/AS1100K/events{/privacy}",
            "received_events_url": "https://api.github.com/users/AS1100K/received_events",
            "type": "User",
            "user_view_type": "public",
            "site_admin": false
        },
        "labels": [],
        "state": "closed",
        "locked": false,
        "assignee": null,
        "assignees": [],
        "milestone": null,
        "comments": 0,
        "created_at": "2025-02-15T15:02:57Z",
        "updated_at": "2025-02-15T15:03:59Z",
        "closed_at": "2025-02-15T15:03:57Z",
        "author_association": "OWNER",
        "sub_issues_summary": {
            "total": 0,
            "completed": 0,
            "percent_completed": 0
        },
        "active_lock_reason": null,
        "draft": false,
        "pull_request": {
            "url": "https://api.github.com/repos/AS1100K/cargo-wiki/pulls/2",
            "html_url": "https://github.com/AS1100K/cargo-wiki/pull/2",
            "diff_url": "https://github.com/AS1100K/cargo-wiki/pull/2.diff",
            "patch_url": "https://github.com/AS1100K/cargo-wiki/pull/2.patch",
            "merged_at": "2025-02-15T15:03:57Z"
        },
        "body": "This isn't the complete implementation, but other support will be added with time as further development is done into this crate.",
        "reactions": {
            "url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/reactions",
            "total_count": 0,
            "+1": 0,
            "-1": 0,
            "laugh": 0,
            "hooray": 0,
            "confused": 0,
            "heart": 0,
            "rocket": 0,
            "eyes": 0
        },
        "timeline_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/timeline",
        "performed_via_github_app": null,
        "state_reason": null,
        "score": 1.0
    });

    let pull_request: PullRequest = serde_json::from_value(json).unwrap();
    let expected = PullRequest {
        html_url: "https://github.com/AS1100K/cargo-wiki/pull/2".into(),
        title: "Add Links for types".into(),
        number: 2,
        updated_at: chrono::DateTime::from_str("2025-02-15T15:03:59Z").unwrap(),
        owner_repo: ("AS1100K".into(), "cargo-wiki".into()),
        state: PullRequestState::Merged,
        owner_avatar_url: "https://avatars.githubusercontent.com/u/117935160?v=4".into(),
    };

    assert_eq!(pull_request, expected);
}

#[test]
fn test_search_res() {
    let json = serde_json::json!({
        "total_count": 71,
        "incomplete_results": false,
        "items": [
            {
                "url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2",
                "repository_url": "https://api.github.com/repos/AS1100K/cargo-wiki",
                "labels_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/labels{/name}",
                "comments_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/comments",
                "events_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/events",
                "html_url": "https://github.com/AS1100K/cargo-wiki/pull/2",
                "id": 2855523382_usize,
                "node_id": "PR_kwDONhMnk86LV35t",
                "number": 2,
                "title": "Add Links for types",
                "user": {
                    "login": "AS1100K",
                    "id": 117935160,
                    "node_id": "U_kgDOBweMOA",
                    "avatar_url": "https://avatars.githubusercontent.com/u/117935160?v=4",
                    "gravatar_id": "",
                    "url": "https://api.github.com/users/AS1100K",
                    "html_url": "https://github.com/AS1100K",
                    "followers_url": "https://api.github.com/users/AS1100K/followers",
                    "following_url": "https://api.github.com/users/AS1100K/following{/other_user}",
                    "gists_url": "https://api.github.com/users/AS1100K/gists{/gist_id}",
                    "starred_url": "https://api.github.com/users/AS1100K/starred{/owner}{/repo}",
                    "subscriptions_url": "https://api.github.com/users/AS1100K/subscriptions",
                    "organizations_url": "https://api.github.com/users/AS1100K/orgs",
                    "repos_url": "https://api.github.com/users/AS1100K/repos",
                    "events_url": "https://api.github.com/users/AS1100K/events{/privacy}",
                    "received_events_url": "https://api.github.com/users/AS1100K/received_events",
                    "type": "User",
                    "user_view_type": "public",
                    "site_admin": false
                },
                "labels": [],
                "state": "closed",
                "locked": false,
                "assignee": null,
                "assignees": [],
                "milestone": null,
                "comments": 0,
                "created_at": "2025-02-15T15:02:57Z",
                "updated_at": "2025-02-15T15:03:59Z",
                "closed_at": "2025-02-15T15:03:57Z",
                "author_association": "OWNER",
                "sub_issues_summary": {
                    "total": 0,
                    "completed": 0,
                    "percent_completed": 0
                },
                "active_lock_reason": null,
                "draft": false,
                "pull_request": {
                    "url": "https://api.github.com/repos/AS1100K/cargo-wiki/pulls/2",
                    "html_url": "https://github.com/AS1100K/cargo-wiki/pull/2",
                    "diff_url": "https://github.com/AS1100K/cargo-wiki/pull/2.diff",
                    "patch_url": "https://github.com/AS1100K/cargo-wiki/pull/2.patch",
                    "merged_at": "2025-02-15T15:03:57Z"
                },
                "body": "This isn't the complete implementation, but other support will be added with time as further development is done into this crate.",
                "reactions": {
                    "url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/reactions",
                    "total_count": 0,
                    "+1": 0,
                    "-1": 0,
                    "laugh": 0,
                    "hooray": 0,
                    "confused": 0,
                    "heart": 0,
                    "rocket": 0,
                    "eyes": 0
                },
                "timeline_url": "https://api.github.com/repos/AS1100K/cargo-wiki/issues/2/timeline",
                "performed_via_github_app": null,
                "state_reason": null,
                "score": 1.0
            },
            {
                "url": "https://api.github.com/repos/rs-workspace/release-butler/issues/8",
                "repository_url": "https://api.github.com/repos/rs-workspace/release-butler",
                "labels_url": "https://api.github.com/repos/rs-workspace/release-butler/issues/8/labels{/name}",
                "comments_url": "https://api.github.com/repos/rs-workspace/release-butler/issues/8/comments",
                "events_url": "https://api.github.com/repos/rs-workspace/release-butler/issues/8/events",
                "html_url": "https://github.com/rs-workspace/release-butler/pull/8",
                "id": 2848428864_usize,
                "node_id": "PR_kwDON1ya3s6K91Py",
                "number": 8,
                "title": "Update README and add documents for marketplace listing",
                "user": {
                    "login": "AS1100K",
                    "id": 117935160,
                    "node_id": "U_kgDOBweMOA",
                    "avatar_url": "https://avatars.githubusercontent.com/u/117935160?v=4",
                    "gravatar_id": "",
                    "url": "https://api.github.com/users/AS1100K",
                    "html_url": "https://github.com/AS1100K",
                    "followers_url": "https://api.github.com/users/AS1100K/followers",
                    "following_url": "https://api.github.com/users/AS1100K/following{/other_user}",
                    "gists_url": "https://api.github.com/users/AS1100K/gists{/gist_id}",
                    "starred_url": "https://api.github.com/users/AS1100K/starred{/owner}{/repo}",
                    "subscriptions_url": "https://api.github.com/users/AS1100K/subscriptions",
                    "organizations_url": "https://api.github.com/users/AS1100K/orgs",
                    "repos_url": "https://api.github.com/users/AS1100K/repos",
                    "events_url": "https://api.github.com/users/AS1100K/events{/privacy}",
                    "received_events_url": "https://api.github.com/users/AS1100K/received_events",
                    "type": "User",
                    "user_view_type": "public",
                    "site_admin": false
                },
                "labels": [],
                "state": "closed",
                "locked": false,
                "assignee": null,
                "assignees": [],
                "milestone": null,
                "comments": 0,
                "created_at": "2025-02-12T14:34:30Z",
                "updated_at": "2025-02-12T14:39:21Z",
                "closed_at": "2025-02-12T14:39:18Z",
                "author_association": "MEMBER",
                "sub_issues_summary": {
                    "total": 0,
                    "completed": 0,
                    "percent_completed": 0
                },
                "active_lock_reason": null,
                "draft": false,
                "pull_request": {
                    "url": "https://api.github.com/repos/rs-workspace/release-butler/pulls/8",
                    "html_url": "https://github.com/rs-workspace/release-butler/pull/8",
                    "diff_url": "https://github.com/rs-workspace/release-butler/pull/8.diff",
                    "patch_url": "https://github.com/rs-workspace/release-butler/pull/8.patch"
                },
                "body": null,
                "reactions": {
                    "url": "https://api.github.com/repos/rs-workspace/release-butler/issues/8/reactions",
                    "total_count": 0,
                    "+1": 0,
                    "-1": 0,
                    "laugh": 0,
                    "hooray": 0,
                    "confused": 0,
                    "heart": 0,
                    "rocket": 0,
                    "eyes": 0
                },
                "timeline_url": "https://api.github.com/repos/rs-workspace/release-butler/issues/8/timeline",
                "performed_via_github_app": null,
                "state_reason": null,
                "score": 1.0
            }
        ]
    });

    let search_res: SearchRes = serde_json::from_value(json).unwrap();
    let expected = SearchRes {
        total_count: 71,
        items: vec![
            PullRequest {
                html_url: "https://github.com/AS1100K/cargo-wiki/pull/2".into(),
                title: "Add Links for types".into(),
                number: 2,
                updated_at: chrono::DateTime::from_str("2025-02-15T15:03:59Z").unwrap(),
                owner_repo: ("AS1100K".into(), "cargo-wiki".into()),
                state: PullRequestState::Merged,
                owner_avatar_url: "https://avatars.githubusercontent.com/u/117935160?v=4".into(),
            },
            PullRequest {
                html_url: "https://github.com/rs-workspace/release-butler/pull/8".into(),
                title: "Update README and add documents for marketplace listing".into(),
                number: 8,
                updated_at: chrono::DateTime::from_str("2025-02-12T14:39:21Z").unwrap(),
                owner_repo: ("rs-workspace".into(), "release-butler".into()),
                state: PullRequestState::Closed,
                owner_avatar_url: "https://avatars.githubusercontent.com/u/117935160?v=4".into(),
            },
        ],
    };

    assert_eq!(search_res, expected)
}
