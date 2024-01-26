//! # Goodomain
//!
//! goodomain is a lib to find TLDs for your new domain, by providing a favorite word you like.
//!
//! See [`find`] API.
#![no_std]

extern crate alloc;

mod data;
mod err;

use alloc::{format, string::String, vec::Vec};
use core::iter::Iterator;

pub use data::*;
pub use err::*;

/// TLDInWord is the result of main [`find`] API. it contains a TLD find in the provided word
/// and it's start..end index.
#[repr(C)]
#[derive(Debug)]
pub struct TLDInWord {
    /// The byte index in provided word.
    pub index: (usize, usize),
    /// The TLD.
    pub tld: &'static str,
}

impl TLDInWord {
    fn new(start: usize, end: usize, tld: &'static str) -> Self {
        Self {
            index: (start, end),
            tld,
        }
    }

    /// Get domain of this result.
    ///
    /// if name is empty, will using `<any>` as a placeholder, e.g. `<any>.com`.
    ///
    /// ## Panic
    ///
    /// Same as [`display`][Self::display].
    pub fn domain(&self, word: &str) -> String {
        let (start, end) = self.index;
        let mut domain = String::with_capacity(word.len() + 7);

        if start == 0 {
            domain.push_str("<any>");
        } else {
            domain.push_str(&word[..start]);
        }
        domain.push('.');
        domain.push_str(&word[start..end]);

        domain
    }

    /// Get path of this result.
    ///
    /// if no path needed, will be empty string without `/`.
    ///
    /// ## Panic
    ///
    /// Same as [`display`][Self::display].
    pub fn path(&self, word: &str) -> String {
        let (_start, end) = self.index;
        let mut path = String::with_capacity(1 + &word[end..].len());
        if end < word.len() {
            path.push('/');
            path.push_str(&word[end..]);
        }
        path
    }

    /// Get full display string of this result. Equal to `domain` + `path`.
    ///
    /// ## Panic
    ///
    /// The `word` parameter should be the same string as you provided when get this `TLDInWord`
    /// from [find] API. Otherwise this function at least gives incorrect result, and may panic.
    pub fn display(&self, word: &str) -> String {
        format!("{}{}", self.domain(word), self.path(word))
    }
}

/// Find good TLDs for a `word` you like.
pub fn find(word: &str) -> Result<impl Iterator<Item = TLDInWord> + '_, FindError> {
    let mut indices = Vec::new();
    for (index, c) in word.char_indices() {
        if c.is_control() || c == '.' || c == '/' {
            return Err(FindError::InvalidWord(StrParamError::InvalidCharacter(c)));
        }
        indices.push(index);
        if indices.len() > 1024 {
            return Err(FindError::InvalidWord(StrParamError::TooLong));
        }
    }
    indices.push(word.len());

    let mut i = 0;
    let mut j = 1;

    Ok(core::iter::from_fn(move || {
        find_iter(word, &indices, &mut i, &mut j)
    }))
}

fn find_iter(word: &str, indices: &[usize], i: &mut usize, j: &mut usize) -> Option<TLDInWord> {
    loop {
        if *j >= indices.len() {
            *i += 1;
            *j = *i + 1;
            if *j >= indices.len() {
                return None;
            }
        }
        let (start, end) = (indices[*i], indices[*j]);
        *j += 1;
        let target = &word[start..end];
        if let Some(tld) = is_tld(target) {
            return Some(TLDInWord::new(start, end, tld));
        }
    }
}
