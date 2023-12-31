# Dispo

[![crates.io](https://img.shields.io/crates/v/dispo.svg)](https://crates.io/crates/dispo)
[![docs.rs](https://docs.rs/dispo/badge.svg)](https://docs.rs/dispo)
[![License](https://img.shields.io/crates/l/dispo.svg)](https://github.com/ynuwenhof/dispo/blob/main/LICENSE-MIT)

A disposable email checker utilizing a [Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter).

* Lightweight
* Probabilistic
* Blazingly fast 🚀
* Backed by [mailchecker's](https://github.com/FGRibreau/mailchecker) email blacklist

> :warning: Due to the nature of [probabilistic data structures](https://en.wikipedia.org/wiki/Bloom_filter),
> [`is_valid`](https://docs.rs/dispo/latest/dispo/fn.is_valid.html) or [`is_valid_domain`](https://docs.rs/dispo/latest/dispo/fn.is_valid_domain.html) might return `false`
> even though a domain is not on the blacklist.

## Usage

```toml
[dependencies]
dispo = "0.1.0"
```

```rust
fn main() {
    let x = dispo::is_valid("alice@example.com");
    assert_eq!(x, true);

    let x = dispo::is_valid_domain("tempmail.de");
    assert_eq!(x, false);
}
```

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/dispo/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/dispo/blob/main/LICENSE-MIT))
