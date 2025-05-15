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
