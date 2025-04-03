use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct QueryPage {
    pub offset: u64,
    pub limit: u64,
}

impl Default for QueryPage {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100
        }
    }
}

impl Display for QueryPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.offset, self.limit)
    }
}

/// creates an iteration of pages to pull the given number of elements
pub fn page_iter(total: u64, page_size: u64) -> impl Iterator<Item=QueryPage> {
    let page_count = total.div_ceil(page_size);
    (0..page_count).into_iter()
        .map(move |page_idx| {
            let offset = page_idx * page_size;
            QueryPage {
                limit: page_size,
                offset,
            }
        })
}
