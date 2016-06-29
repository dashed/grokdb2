/* local imports */

use types::{Page, PerPage, SortOrder, Count};

////////////////////////////////////////////////////////////////////////////////

pub fn num_of_pages(items: Count, per_page: PerPage) -> Count {
    let pages = (items as f64 / per_page as f64).ceil() as Count;
    return pages;
}
