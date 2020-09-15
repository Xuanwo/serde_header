mod de;
mod error;

pub use error::{Error, Result};

pub fn from_http_header_map<'de, T>(h: &'de http::header::HeaderMap) -> Result<T>
where
    T: serde::de::Deserialize<'de>,
{
    T::deserialize(de::Deserializer::from_http_header_map(h))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test() {
        #[derive(Deserialize, Debug)]
        struct Test {
            content_length: i64,
            content_length1: i64,
        }

        let mut h = http::header::HeaderMap::new();
        h.insert("content_length", "100".parse().unwrap());
        h.insert("content_length1", "1020".parse().unwrap());

        let t: Test = from_http_header_map(&h).unwrap();

        println!("{:?}", &t)
    }
}