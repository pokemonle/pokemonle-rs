use aide::OperationIo;
use serde::{Deserialize, Serialize};

use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, OperationIo, JsonSchema)]
pub struct ResourceId {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, OperationIo, JsonSchema)]
pub struct VersionGroup {
    pub version_group: i32,
}

fn default_language() -> i32 {
    12
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, OperationIo, JsonSchema)]
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
    pub page: u64,
    #[validate(range(min = 10, max = 100))]
    pub per_page: u64,
}

impl Default for PaginateQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 25,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, OperationIo, JsonSchema)]
pub struct FlavorParams(ResourceId, Language);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, OperationIo, JsonSchema)]
pub enum FlavorResource {
    Item {
        #[serde(flatten)]
        params: FlavorParams,
    },
    Ability {
        #[serde(flatten)]
        params: FlavorParams,
    },
    Move {
        id: i32,
        version_group: i32,
    },
    PokemonSpecies {
        id: i32,
        version_group: i32,
    },
}
