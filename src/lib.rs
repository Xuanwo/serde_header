mod de;
mod ser;
mod error;

#[cfg(feature = "crate_http")]
mod crate_http;

#[cfg(feature = "crate_http")]
use crate_http::{HeaderMap, HeaderMapOwned};

pub use error::{Error, Result};

#[cfg(feature = "crate_http")]
pub fn from_header_map<'de, T>(h: &'de http::HeaderMap) -> Result<T>
    where
        T: serde::de::Deserialize<'de>,
{
    T::deserialize(de::Deserializer::from_header_map(h))
}

#[cfg(feature = "crate_http")]
pub fn to_header_map<T>(v: &T) -> Result<http::HeaderMap> where T: serde::ser::Serialize {
    ser::to_header_map(v)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_from_header_map() {
        #[derive(Deserialize, Debug)]
        struct Test {
            content_length: i64,
            content_length1: i64,
        }

        let mut h = http::header::HeaderMap::new();
        h.insert("content_length", "100".parse().unwrap());
        h.insert("content_length1", "1020".parse().unwrap());

        let t: Test = from_header_map(&h).unwrap();

        println!("{:?}", &t)
    }

    #[test]
    fn test_to_header_map() {
        #[derive(Serialize, Debug)]
        struct Test {
            content_length: i64,
            content_length1: i64,
        }

        let mut v = Test { content_length: 100, content_length1: 200 };

        let t = to_header_map(&v).unwrap();

        println!("{:?}", &t)
    }
}
