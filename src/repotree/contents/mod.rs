pub mod view;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct INode {
    pub name: String,
    pub path: String,
    pub url: String,
    pub size: u32,
    #[serde(flatten)]
    pub content: INodeContent
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum INodeContent {
    #[serde(rename = "file")]
    File {
        content: Option<String>,
    },
    #[serde(rename = "dir")]
    Dir {
        entries: Option<Vec<INode>>
    }
}
