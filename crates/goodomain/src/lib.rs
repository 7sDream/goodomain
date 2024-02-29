//! # Goodomain
//!
//! goodomain is a lib to find TLDs for your new domain, by providing a favorite word you like.
//!
//! See [`find`] API.
#![no_std]

extern crate alloc;

mod data;
mod err;

use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Display, Formatter},
    iter::Iterator,
};

pub use data::*;
pub use err::*;

/// TLDInWord is the result of main [`find`] API. it contains a TLD find in the provided word
/// and it's start..end index.
#[repr(C)]
#[derive(Debug)]
pub struct TLDInWord<'a> {
    /// The Word
    pub word: &'a str,
    /// The byte index in provided word.
    pub index: (usize, usize),
}

impl<'a> TLDInWord<'a> {
    /// This method is not pubic, because there are some requirements that arguments must follow:
    ///
    /// - start < end
    /// - word.len() >= end
    /// - `start` and `end` must at character boundary in `word`
    fn new(word: &'a str, start: usize, end: usize) -> Self {
        Self {
            word,
            index: (start, end),
        }
    }

    /// Get tld of this result.
    pub fn tld(&self) -> &'a str {
        let (start, end) = self.index;
        &self.word[start..end]
    }

    /// Get domain of this result.
    ///
    /// if name is empty, will using `<any>` as a placeholder, e.g. `<any>.com`.
    pub fn domain(&self) -> String {
        let (start, _end) = self.index;
        let mut domain = String::with_capacity(self.word.len() + 7);

        if start == 0 {
            domain.push_str("<any>");
        } else {
            domain.push_str(&self.word[..start]);
        }
        domain.push('.');
        domain.push_str(self.tld());

        domain
    }

    /// Get path of this result.
    ///
    /// if no path needed, will be empty string without `/`.
    pub fn path(&self) -> String {
        let (_start, end) = self.index;
        let mut path = String::with_capacity(1 + &self.word[end..].len());
        if end < self.word.len() {
            path.push('/');
            path.push_str(&self.word[end..]);
        }
        path
    }
}

impl<'a> Display for TLDInWord<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.domain(), self.path()))
    }
}

/// Find good TLDs for a `word` you like.
pub fn find(word: &str) -> Result<impl Iterator<Item = TLDInWord<'_>>, FindError> {
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

fn find_iter<'s>(
    word: &'s str, indices: &[usize], i: &mut usize, j: &mut usize,
) -> Option<TLDInWord<'s>> {
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
        if is_tld(target) {
            return Some(TLDInWord::new(word, start, end));
        }
    }
}
