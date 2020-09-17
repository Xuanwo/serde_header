use std::fmt;

use serde::de;
use serde::de::{DeserializeSeed, Expected, IntoDeserializer, Unexpected, Visitor};
use serde::forward_to_deserialize_any;

use super::{Error, Result, HeaderMap};
use serde::de::Error as de_error;

pub struct Deserializer<'de> {
    inner: super::HeaderMap<'de>,
}

impl<'de> Deserializer<'de> {
    #[cfg(feature = "crate_http")]
    pub fn from_header_map(h: &'de http::HeaderMap) -> Self {
        Deserializer {
            inner: HeaderMap::new(h),
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
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
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
            Some(v) => {
                self.index += 1;
                seed.deserialize(MapDeserializer {
                    map: self.map,
                    field: v,
                })
            }
        }
    }
}

struct MapDeserializer<'de> {
    map: HeaderMap<'de>,
    field: &'static str,
}

macro_rules! forward_to_value_deserializer {
    ($($func:ident)*) => {$(
        #[inline]
        fn $func<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            match self.map.get(self.field)? {
                None => Err(Error::missing_field(self.field)),
                Some(v) => ValueDeserializer(v).$func(visitor)
            }
        })*
    };
}

impl<'de> de::Deserializer<'de> for MapDeserializer<'de> {
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

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.map.contains(self.field) {
            false => visitor.visit_none(),
            true => match self.map.get(self.field)? {
                None => Err(Error::missing_field(self.field)),
                Some(v) => visitor.visit_some(ValueDeserializer(v))
            },
        }
    }

    forward_to_value_deserializer! {
        deserialize_i8 deserialize_i16 deserialize_i32 deserialize_i64
        deserialize_u8 deserialize_u16 deserialize_u32 deserialize_u64
        deserialize_f32 deserialize_f64 deserialize_char deserialize_str
        deserialize_string deserialize_bytes deserialize_byte_buf
    }

    forward_to_deserialize_any! {
        bool unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

macro_rules! parse_digit_helper {
    ($func:ident, $ty:ty, $parse:ident) => {
        fn $func<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            match self.0.parse::<$ty>() {
                Err(_) => Err(Error::invalid_value(
                    Unexpected::Str(self.0),
                    &"digit only",
                )),
                Ok(v) => visitor.$parse(v),
            }
        }
    };
}

struct ValueDeserializer<'de>(&'de str);

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
        if self.0.chars().count() != 1 {
            return Err(Error::invalid_value(Unexpected::Str(self.0), &"a char"));
        }
        visitor.visit_char(self.0.chars().next().unwrap())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_str(self.0)
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_string(self.0.to_string())
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_bytes(self.0.as_bytes())
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.0.as_bytes().to_vec())
    }

    forward_to_deserialize_any! {
        bool option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    #[cfg(feature = "crate_http")]
    fn test() {
        #[derive(Deserialize, Debug)]
        struct Test {
            content_length: i64,
            content_type: Option<String>,
        }

        let mut h = http::header::HeaderMap::new();
        h.insert("content_length", "100".parse().unwrap());
        h.insert("content_type", "ABC".parse().unwrap());

        let d = Deserializer {
            inner: HeaderMap::new(&h),
        };

        let t: Test = serde::Deserialize::deserialize(d).unwrap();

        println!("{:?}", &t)
    }
}
