pub type Count = u64; // type for counting things

// pagination
// TODO: need compile-time check to ensure >= 1 constraint (rust doesn't support this yet)
pub type Page = u64;
pub type PerPage = u64;

pub enum SortOrder {
    Ascending,
    Descending
}

pub type UnixTimestamp = i64;

// TODO: migrate some types from route.rs to here
