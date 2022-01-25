Monostate
=========

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/monostate-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/monostate)
[<img alt="crates.io" src="https://img.shields.io/crates/v/monostate.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/monostate)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-monostate-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/monostate)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/monostate/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/monostate/actions?query=branch%3Amaster)

This library implements a type macro for a zero-sized type that is Serde
deserializable only from one specific value.

```toml
[dependencies]
monostate = "0.0"
```

<br>

## Examples

```rust
use monostate::MustBe;
use serde::Deserialize;

#[derive(Deserialize)]
struct Example {
    kind: MustBe!("success"),
    code: MustBe!(200),
}
```

The above struct would deserialize from `{"kind":"success", "code":200}` in
JSON, but would fail the deserialization if "kind" or "code" were any other
value.

This can sometimes be helpful in processing untagged enums in which the variant
indentification is more convoluted than what is handled by Serde's externally
tagged and internally tagged representations, for example because the variant
tag has an inconsistent type or key.

```rust
use monostate::MustBe;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success {
        success: MustBe!(true),
    },
    Error {
        kind: MustBe!("error"),
        message: String,
    },
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
