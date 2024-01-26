use alloc::collections::BTreeSet;

use once_cell::race::OnceBox;

/// TLD data file version.
///
/// In date format like `20230101`.
pub static TLD_VERSION: u64 = include!(concat!(env!("OUT_DIR"), "/tld_version.in"));

/// All TLD list.
pub static TLD_LIST: [&str; include!(concat!(env!("OUT_DIR"), "/tld_count.in"))] =
    include!(concat!(env!("OUT_DIR"), "/tld_list.in"));

static TLD_SET: OnceBox<BTreeSet<&'static str>> = OnceBox::new();

/// Checks if a string in invalid TLD.
///
/// If true, will return the TLD's static string.
///
/// Faster then iterate through the `TLD_LIST`.
pub fn is_tld(text: &str) -> Option<&'static str> {
    TLD_SET
        .get_or_init(|| alloc::boxed::Box::new(TLD_LIST.iter().copied().collect()))
        .get(text)
        .copied()
}
