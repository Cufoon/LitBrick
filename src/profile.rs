use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoItem {
    pub name: String,
    pub path: String,
    #[serde(flatten)]
    pub data: RepoItemData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum RepoItemData {
    #[serde(rename = "git")]
    Git(GitConfig),
    #[serde(rename = "http")]
    Http(HttpConfig),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitConfig {
    pub url: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpConfig {
    pub url: String,
}
