use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMargin {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
    pub header: i32,
    pub footer: i32,
    pub gutter: i32,
}
