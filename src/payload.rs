//! src/payload.rs
//! This mod contains the payload struct of Github-Webhook
#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    zen: String,
    hook_id: u64,
    hook: Hook,
    repository: Repository,
    sender: Sender,
}

#[derive(Debug, Deserialize)]
pub struct Sender {
    login: String,
    id: u64,
    node_id: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    id: u64,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
    owner: Owner,
    html_url: String,
    description: String,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    pushed_at: DateTime<Utc>,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: Option<String>,
    size: usize,
    stargazers_count: usize,
    watchers_count: usize,
    language: Option<String>,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    has_discussions: bool,
    forks_count: usize,
    mirror_url: Option<String>,
    archived: bool,
    disabled: bool,
    open_issues_count: usize,
    license: Option<String>,
    allow_forking: bool,
    is_template: bool,
    web_commit_signoff_required: bool,
    topics: Vec<String>,
    visibility: String,
    forks: usize,
    open_issues: usize,
    watchers: usize,
    default_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct Hook {
    r#type: String,
    id: u64,
    name: String,
    active: bool,
    events: Vec<String>,
    config: Config,
    updated_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    url: String,
    test_url: String,
    ping_url: String,
    deliveries_url: String,
    last_response: LastResponse,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    content_type: String,
    insecure_ssl: String,
    secret: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct LastResponse {
    code: Option<String>,
    status: String,
    message: Option<String>,
}
