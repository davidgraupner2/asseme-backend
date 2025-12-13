use serde::{Deserialize, Serialize};

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_PER_PAGE: i64 = 20;
const MAX_PER_PAGE: i64 = 100;

#[derive(Deserialize)]
pub struct PaginationQuery {
    page: Option<i64>,
    per_page: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
    pub offset: i64,
}

impl PaginationQuery {
    pub fn pagination(&self) -> Pagination {
        let page = self.page.unwrap_or(DEFAULT_PAGE).max(1);
        let per_page = self
            .per_page
            .unwrap_or(DEFAULT_PER_PAGE)
            .clamp(1, MAX_PER_PAGE);
        let offset = (page - 1) * per_page;

        Pagination {
            page,
            per_page,
            offset,
        }
    }
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn new(pagination: &Pagination, total: i64) -> Self {
        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;
        Self {
            page: pagination.page,
            per_page: pagination.per_page,
            total,
            total_pages,
        }
    }
}
