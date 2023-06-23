//! A disposable email checker utilizing a [Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter).
//!
//! * Lightweight
//! * Probabilistic
//! * Blazingly fast 🚀
//! * Backed by [mailchecker's](https://github.com/FGRibreau/mailchecker) email blacklist
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! toml = "0.1.0"
//! ```
//!
//! ```rust
//! let x = dispo::is_valid("alice@example.com");
//! assert_eq!(x, true);
//!
//! let x = dispo::is_valid_domain("tempmail.de");
//! assert_eq!(x, false);
//! ```

use email_address::EmailAddress;
use std::str::FromStr;

mod bloom;

include!(concat!(env!("OUT_DIR"), "/dispo.rs"));

/// Returns `true` if the email has a valid format and is not disposable.
///
/// # Examples
///
/// ```
/// let x = dispo::is_valid("alice@example.com");
/// assert_eq!(x, true);
///
/// let x = dispo::is_valid("alice@tempmail.de");
/// assert_eq!(x, false);
///
/// let x = dispo::is_valid("alice.example.com");
/// assert_eq!(x, false);
/// ```
pub fn is_valid(email: &str) -> bool {
    EmailAddress::from_str(email).map_or(false, |e| is_valid_domain(e.domain()))
}

/// Returns `true` if the domain is not disposable, doesn't
/// check if the domain itself has a valid format.
///
/// # Examples
///
/// ```
/// let x = dispo::is_valid_domain("example.com");
/// assert_eq!(x, true);
///
/// let x = dispo::is_valid_domain("tempmail.de");
/// assert_eq!(x, false);
///
/// let x = dispo::is_valid_domain("tempmail");
/// assert_eq!(x, true);
/// ```
pub fn is_valid_domain(domain: &str) -> bool {
    !BLOOM.contains(&domain)
}
