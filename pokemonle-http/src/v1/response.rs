use aide::{
    transform::TransformOperation,
    // openapi::{Operation, Response},
    OperationIo,

    OperationOutput, // OperationOutput,
};
use axum::{extract::Path, Json};
use pokemonle_trait::StructName;
use schemars::JsonSchema;
// use axum::{extract::rejection::JsonRejection, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, OperationIo, JsonSchema)]
#[aide(output, json_schema)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub total: usize,
}

pub fn list_items_docs<T>(op: TransformOperation) -> TransformOperation
where
    T: JsonSchema + Serialize + StructName,
{
    T::tags()
        .iter()
        .fold(op, |acc, tag| acc.tag(tag))
        .description(&format!("Get a list of {}", T::struct_name()))
        .response_with::<200, Json<ListResponse<T>>, _>(|res| {
            res.description("example")
            // .example(ListResponse {
            //     data: vec![T::example()],
            //     total: 1,
            // })
        })
        .response_with::<404, (), _>(|res| {
            res.description(&format!("{} not found", T::struct_name()))
        })
}

pub fn get_item_by_id_docs<T>(op: TransformOperation) -> TransformOperation
where
    T: StructName + OperationOutput + Serialize + JsonSchema,
    <T as OperationOutput>::Inner: Serialize + From<T>,
{
    let struct_name = T::struct_name();

    T::tags()
        .iter()
        .fold(op, |acc, tag| acc.tag(tag))
        .input::<Path<i32>>()
        .description(&format!("Get a {} by id", T::struct_name()))
        .parameter::<i32, _>("id", |tp| tp.description("ID"))
        .response_with::<200, Json<T>, _>(|res| res.description(&format!("{} found", struct_name)))
        .response_with::<404, (), _>(|res| res.description(&format!("{} not found", struct_name)))
}
