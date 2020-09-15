# serde_header

Strongly typed HTTP Header library for Rust, built upon serde

## Quick start

```rust
use serde_header::from_http_header_map;

#[derive(Deserialize, Debug)]
struct Example {
    content_length: i64,
    content_type: String,
}

// let mut h = http::header::HeaderMap::new();
// h.insert("content_length", "100".parse().unwrap());
// h.insert("content_type", "application/json".parse().unwrap());

let t: Example = from_http_header_map(&h).unwrap();

println!("{:?}", &t)
```