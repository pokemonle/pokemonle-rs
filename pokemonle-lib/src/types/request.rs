use aide::OperationIo;
use serde::{Deserialize, Serialize};

use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
pub struct ResourceId {
    pub id: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct VersionGroup {
    pub version_group: i32,
}

fn default_language() -> i32 {
    12
}

#[derive(Deserialize, JsonSchema)]
pub struct Language {
    #[serde(default = "default_language")]
    pub lang: i32,
}

impl Default for Language {
    fn default() -> Self {
        Self {
            lang: default_language(),
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, OperationIo, JsonSchema)]
#[serde(default)]
pub struct PaginateQuery {
    pub page: i64,
    #[validate(range(min = 10, max = 100))]
    pub per_page: i64,
}

impl Default for PaginateQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 25,
        }
    }
}
