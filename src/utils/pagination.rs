use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    // Previous page url
    pub previous: Option<String>,
    // Next page url
    pub next: Option<String>,
    // Previous page number
    pub previous_page: Option<u32>,
    // Next page number
    pub next_page: Option<u32>,
    pub total_pages: u32,
    pub total_items: i32,
    pub size: u32,
    // Should this be `page` instead of start? (Do we even need it?)
    // start: u32,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        const DEFAULT_PAGE: u32 = 1;
        const DEFAULT_PAGE_SIZE: u32 = 2;

        PaginationQuery {
            page: Some(DEFAULT_PAGE),
            size: Some(DEFAULT_PAGE_SIZE),
        }
    }
}

pub fn sanitize_pagination_query(query: PaginationQuery) -> PaginationQuery {
    let page = query
        .page
        .unwrap_or(PaginationQuery::default().page.unwrap());
    let size = query
        .size
        .unwrap_or(PaginationQuery::default().size.unwrap());

    PaginationQuery {
        page: Some(page),
        size: Some(size),
    }
}

pub fn has_next_page(total: u32, current_page: u32) -> bool {
    if total <= current_page {
        return false;
    }
    true
}

pub fn has_previous_page(current_page: u32) -> bool {
    if current_page <= 1 {
        return false;
    }
    true
}

#[cfg(test)]
mod pagination_tests {
    use super::{has_next_page, has_previous_page, sanitize_pagination_query, PaginationQuery};

    #[test]
    fn empty_query_pagination() {
        let empty_query = PaginationQuery {
            page: None,
            size: None,
        };

        let pagination_result = sanitize_pagination_query(empty_query);

        let expected_pagination = PaginationQuery {
            page: PaginationQuery::default().page,
            size: PaginationQuery::default().size,
        };

        assert_eq!(pagination_result, expected_pagination);
    }

    #[test]
    fn valid_query_pagination() {
        let valid_query = PaginationQuery {
            page: Some(4),
            size: Some(10),
        };

        let pagination_result = sanitize_pagination_query(valid_query);

        let expected_pagination = PaginationQuery {
            page: Some(4),
            size: Some(10),
        };

        assert_eq!(pagination_result, expected_pagination);
    }

    #[test]
    fn missing_page_from_query_pagination() {
        let missing_page_query = PaginationQuery {
            page: None,
            size: Some(10),
        };

        let pagination_result = sanitize_pagination_query(missing_page_query);

        let expected_pagination = PaginationQuery {
            page: PaginationQuery::default().page,
            size: Some(10),
        };

        assert_eq!(pagination_result, expected_pagination);
    }

    #[test]
    fn missing_size_from_query_pagination() {
        let missing_size_query = PaginationQuery {
            page: Some(10),
            size: None,
        };

        let pagination_result = sanitize_pagination_query(missing_size_query);

        let expected_pagination = PaginationQuery {
            page: Some(10),
            size: PaginationQuery::default().size,
        };

        assert_eq!(pagination_result, expected_pagination);
    }

    #[test]
    fn check_has_next_page() {
        assert_eq!(has_next_page(10, 5), true);
        assert_eq!(has_next_page(10, 10), false);
        assert_eq!(has_next_page(10, 12), false);
    }

    #[test]
    fn check_has_previous_page() {
        assert_eq!(has_previous_page(10), true);
        assert_eq!(has_previous_page(1), false);
        assert_eq!(has_previous_page(0), false);
    }
}
