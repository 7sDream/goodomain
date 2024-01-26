/// FindError is errors for [`find`][crate::find] API.
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, displaydoc::Display)]
pub enum FindError {
    /// parameter `word` is invalid: {0}
    InvalidWord(StrParamError),
}

/// Error reason for a string parameters.
#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, displaydoc::Display)]
pub enum StrParamError {
    /// it is empty
    Empty,
    /// it is too long
    TooLong,
    /// it contains invalid character '{0}'
    InvalidCharacter(char),
}
