use serde::{ser, Serialize};

use super::{Error, Result};
use std::fmt::Display;
use serde::ser::Impossible;
use std::str::from_utf8;

pub struct Serializer {
    current: Option<String>,
    output: super::HeaderMapOwned,
}

#[cfg(feature = "crate_http")]
pub fn to_header_map<T>(value: &T) -> Result<http::HeaderMap> where
    T: Serialize, {
    let mut serializer = Serializer {
        current: None,
        output: super::HeaderMapOwned::new(http::HeaderMap::new()),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output.take())
}


impl ser::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v.to_string().as_str());

        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), v);

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.output.insert(self.current.as_ref().unwrap().as_str(), from_utf8(v)?);

        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        self.current = Some(key.to_string());
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    #[cfg(feature = "crate_http")]
    fn test() {
        #[derive(Serialize, Debug)]
        struct Test {
            content_length: i64,
            x: bool,
            test: Option<u64>,
        }

        let v = Test { content_length: 100, x: true, test: Some(123) };

        let t = to_header_map(&v);

        println!("{:?}", &t.unwrap())
    }
}