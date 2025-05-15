use aide::OperationIo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, OperationIo, JsonSchema)]
#[aide(output, json_schema)]
pub struct PaginatedResource<T> {
    pub data: Vec<T>,
    // Current page number
    pub page: i64,
    // Number of items per page
    pub per_page: i64,
    // Total number of pages
    pub total_pages: i64,
    // Total number of items
    pub total_items: i64,
}

impl<T> PaginatedResource<T> {
    pub fn new(data: Vec<T>, page: i64, per_page: i64, total_pages: i64, total_items: i64) -> Self {
        PaginatedResource {
            data,
            page,
            per_page,
            total_pages,
            total_items,
        }
    }

    pub fn new_from_vec(data: Vec<T>) -> Self {
        let length = data.len() as i64;
        PaginatedResource {
            data,
            page: 1,
            per_page: length,
            total_pages: 1,
            total_items: length,
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> PaginatedResource<U> {
        PaginatedResource {
            data: self.data.into_iter().map(f).collect(),
            page: self.page,
            per_page: self.per_page,
            total_pages: self.total_pages,
            total_items: self.total_items,
        }
    }

    pub fn map_data<U>(self, f: impl Fn(Vec<T>) -> Vec<U>) -> PaginatedResource<U> {
        PaginatedResource {
            data: f(self.data),
            page: self.page,
            per_page: self.per_page,
            total_pages: self.total_pages,
            total_items: self.total_items,
        }
    }
}
