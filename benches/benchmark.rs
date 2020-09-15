use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde::Deserialize;
use serde_header::from_http_header_map;

#[derive(Deserialize, Debug)]
struct Test {
    test_i8: i8,
    test_i16: i16,
    test_i32: i32,
    test_i64: i64,
    test_i128: i128,
    test_u8: u8,
    test_u16: u16,
    test_u32: u32,
    test_u64: u64,
    test_u128: u128,

    test_f32: f32,
    test_f64: f64,

    test_string: String,
    test_char: char,
}

fn test(c: &mut Criterion) {
    let mut h = http::header::HeaderMap::new();
    h.insert("test_i8", "100".parse().unwrap());
    h.insert("test_i16", "100".parse().unwrap());
    h.insert("test_i32", "100".parse().unwrap());
    h.insert("test_i64", "100".parse().unwrap());
    h.insert("test_i128", "100".parse().unwrap());
    h.insert("test_u8", "100".parse().unwrap());
    h.insert("test_u16", "100".parse().unwrap());
    h.insert("test_u32", "100".parse().unwrap());
    h.insert("test_u64", "100".parse().unwrap());
    h.insert("test_u128", "100".parse().unwrap());

    h.insert("test_f32", "100".parse().unwrap());
    h.insert("test_f64", "100".parse().unwrap());

    h.insert("test_string", "Hello, world!".parse().unwrap());
    h.insert("test_char", "秦".parse().unwrap());

    c.bench_function("test", |b| {
        b.iter(|| {
            let t: Test = from_http_header_map(&h).unwrap();
        });
    });
}

criterion_group!(benches, test);
criterion_main!(benches);
