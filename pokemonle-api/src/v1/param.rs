use serde::Deserialize;

use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
pub(crate) struct Resource {
    pub id: i32,
}

#[derive(Deserialize, JsonSchema)]
pub(crate) struct VersionGroup {
    pub version_group: i32,
}

fn default_language() -> i32 {
    12
}

#[derive(Deserialize, JsonSchema)]
pub(crate) struct Language {
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
pub(crate) struct SearchQuery {
    pub(crate) q: Option<String>,
}
