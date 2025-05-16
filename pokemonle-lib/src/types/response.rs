use aide::OperationIo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, OperationIo, JsonSchema)]
#[aide(output, json_schema)]
pub struct PaginatedResource<T> {
    pub data: Vec<T>,
    // Current page number
    pub page: u64,
    // Number of items per page
    pub per_page: u64,
    // Total number of pages
    pub total_pages: u64,
    // Total number of items
    pub total_items: u64,
}

impl<T> PaginatedResource<T> {
    pub fn new(data: Vec<T>, page: u64, per_page: u64, total_pages: u64, total_items: u64) -> Self {
        PaginatedResource {
            data,
            page,
            per_page,
            total_pages,
            total_items,
        }
    }

    pub fn new_from_vec(data: Vec<T>) -> Self {
        let length = data.len() as u64;
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

    pub fn map_data<U>(self, f: impl FnOnce(Vec<T>) -> Vec<U>) -> PaginatedResource<U> {
        PaginatedResource {
            data: f(self.data),
            page: self.page,
            per_page: self.per_page,
            total_pages: self.total_pages,
            total_items: self.total_items,
        }
    }

    pub fn map_data_iter<U>(self, f: impl FnMut(T) -> U) -> PaginatedResource<U> {
        PaginatedResource {
            data: self.data.into_iter().map(f).collect(),
            page: self.page,
            per_page: self.per_page,
            total_pages: self.total_pages,
            total_items: self.total_items,
        }
    }
}
