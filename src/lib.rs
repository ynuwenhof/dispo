use std::str::FromStr;
use email_address::EmailAddress;

mod bloom;

include!(concat!(env!("OUT_DIR"), "/dispo.rs"));

pub fn is_valid(email: &str) -> bool {
    EmailAddress::from_str(email).map_or(false, |e| is_valid_domain(e.domain()))
}

pub fn is_valid_domain(domain: &str) -> bool {
    !BLOOM.contains(&domain)
}
