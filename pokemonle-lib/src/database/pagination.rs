use aide::OperationIo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_PER_PAGE: i64 = 25;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, OperationIo, JsonSchema)]
#[serde(default)]
pub struct Paginated {
    pub page: i64,
    #[validate(range(min = 10, max = 100))]
    pub per_page: i64,
}

impl Default for Paginated {
    fn default() -> Self {
        Paginated {
            page: DEFAULT_PAGE,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl Paginated {
    pub fn new(page: i64, per_page: i64) -> Self {
        Paginated { page, per_page }
    }

    pub fn limit(self) -> i64 {
        self.per_page
    }

    pub fn offset(self) -> i64 {
        (self.page - 1) * self.per_page
    }

    pub fn pages(self, total: i64) -> i64 {
        if self.per_page == 0 {
            return 1;
        }
        if total == 0 {
            return 0;
        }
        (total as f64 / self.per_page as f64).ceil() as i64
    }
}

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
