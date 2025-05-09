use aide::OperationIo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, OperationIo, JsonSchema)]
#[aide(output, json_schema)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub total: usize,
}
