use std::fmt;
use std::str::from_utf8;

use serde::de;
use serde::de::{DeserializeSeed, Expected, IntoDeserializer, Unexpected, Visitor};
use serde::forward_to_deserialize_any;

use super::{Error, Result};
use serde::de::Error as de_error;

pub struct Deserializer<'de> {
    inner: HeaderMap<'de>,
}

impl<'de> Deserializer<'de> {
    pub fn from_http_header_map(h: &'de http::header::HeaderMap) -> Self {
        Deserializer {
            inner: HeaderMap::CrateHttp(h),
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let _ = visitor;
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let map = MapAccess {
            index: 0,
            fields,
            map: self.inner,
        };

        visitor.visit_map(map)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

enum HeaderMap<'de> {
    CrateHttp(&'de http::header::HeaderMap),
}

impl<'de> HeaderMap<'de> {
    fn get(&self, key: &str) -> Result<Option<&'de str>> {
        match self {
            HeaderMap::CrateHttp(v) => match v.get(key) {
                None => Ok(None),
                Some(v) => {
                    let s = from_utf8(v.as_bytes());

                    s.map(Some).map_err(|_| {
                        de_error::invalid_value(
                            Unexpected::Bytes(v.as_bytes()),
                            &"valid utf-8 chars",
                        )
                    })
                }
            },
        }
    }
}

struct ExpectedInSeq(usize);

impl de::Expected for ExpectedInSeq {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 1 {
            write!(formatter, "1 element in sequence")
        } else {
            write!(formatter, "{} elements in sequence", self.0)
        }
    }
}

struct MapAccess<'de> {
    index: usize,
    fields: &'static [&'static str],
    map: HeaderMap<'de>,
}

impl<'de> de::MapAccess<'de> for MapAccess<'de> {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where
            T: DeserializeSeed<'de>,
    {
        if self.index >= self.fields.len() {
            return Ok(None);
        }

        seed.deserialize(self.fields.get(self.index).unwrap().into_deserializer())
            .map(Some)
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value>
        where
            T: DeserializeSeed<'de>,
    {
        match self.fields.get(self.index) {
            None => Err(Error::invalid_length(
                self.index,
                &ExpectedInSeq(self.fields.len()),
            )),
            Some(v) => match self.map.get(v)? {
                None => Err(Error::missing_field(v)),
                Some(v) => {
                    self.index += 1;
                    seed.deserialize(ValueDeserializer { value: v })
                }
            },
        }
    }
}

struct ValueDeserializer<'de> {
    value: &'de str,
}

macro_rules! parse_digit_helper {
    ($func:ident, $ty:ty, $parse:ident) => {
        fn $func<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            match self.value.parse::<$ty>() {
                Err(_) => Err(Error::invalid_value(
                    Unexpected::Str(self.value),
                    &"digit only",
                )),
                Ok(v) => visitor.$parse(v),
            }
        }
    };
}

impl<'de> de::Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        Err(Error::custom(format_args!(
            "unsupported type: {}",
            &visitor as &dyn Expected
        )))
    }

    parse_digit_helper!(deserialize_i8, i8, visit_i8);
    parse_digit_helper!(deserialize_i16, i16, visit_i16);
    parse_digit_helper!(deserialize_i32, i32, visit_i32);
    parse_digit_helper!(deserialize_i64, i64, visit_i64);
    parse_digit_helper!(deserialize_i128, i128, visit_i128);
    parse_digit_helper!(deserialize_u8, u8, visit_u8);
    parse_digit_helper!(deserialize_u16, u16, visit_u16);
    parse_digit_helper!(deserialize_u32, u32, visit_u32);
    parse_digit_helper!(deserialize_u64, u64, visit_u64);
    parse_digit_helper!(deserialize_u128, u128, visit_u128);
    parse_digit_helper!(deserialize_f32, f32, visit_f32);
    parse_digit_helper!(deserialize_f64, f64, visit_f64);

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        if self.value.chars().count() != 1 {
            return Err(Error::invalid_value(Unexpected::Str(self.value), &"a char"));
        }
        visitor.visit_char(self.value.chars().next().unwrap())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_str(self.value)
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_string(self.value.to_string())
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_bytes(self.value.as_bytes())
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.value.as_bytes().to_vec())
    }

    forward_to_deserialize_any! {
        bool option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[derive(Deserialize, Debug)]
        struct Test {
            content_length: i64,
            content_type: String,
        }

        let mut h = http::header::HeaderMap::new();
        h.insert("content_length", "100".parse().unwrap());
        h.insert("content_type", "ABC".parse().unwrap());

        let d = Deserializer {
            inner: HeaderMap::CrateHttp(&h),
        };

        let t: Test = serde::Deserialize::deserialize(d).unwrap();

        println!("{:?}", &t)
    }
}
