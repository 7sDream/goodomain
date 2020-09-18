use once_cell::unsync::Lazy;
use std::collections::HashSet;

pub const VERSION: u64 = include!(concat!(env!("OUT_DIR"), "/tld_version.in"));
pub const TLD_COUNT: usize = include!(concat!(env!("OUT_DIR"), "/tld_count.in"));
pub const TLD_LIST: Lazy<HashSet<&'static [char]>> = Lazy::new(|| include!(concat!(env!("OUT_DIR"), "/tld_list.in")));
