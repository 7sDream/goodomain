use alloc::collections::BTreeSet;

use once_cell::race::OnceBox;
use unicase::Ascii;

/// TLD data file version.
///
/// In date format like `20230101`.
pub static TLD_VERSION: u64 = include!(concat!(env!("OUT_DIR"), "/tld_version.in"));

/// All TLD list.
pub static TLD_LIST: [&str; include!(concat!(env!("OUT_DIR"), "/tld_count.in"))] =
    include!(concat!(env!("OUT_DIR"), "/tld_list.in"));

fn tld_set() -> &'static BTreeSet<Ascii<&'static str>> {
    static TLD_SET: OnceBox<BTreeSet<Ascii<&str>>> = OnceBox::new();

    TLD_SET
        .get_or_init(|| alloc::boxed::Box::new(TLD_LIST.iter().copied().map(Ascii::new).collect()))
}

/// Checks if a string in invalid TLD.
pub fn is_tld(text: &str) -> bool {
    tld_set().contains(&Ascii::new(text))
}
