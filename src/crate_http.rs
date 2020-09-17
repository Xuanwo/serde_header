use std::str::from_utf8;

use super::Result;

#[derive(Copy, Clone)]
pub struct HeaderMap<'de>(&'de http::HeaderMap);

impl<'de> HeaderMap<'de> {
    pub fn new(h: &'de http::HeaderMap) -> Self {
        HeaderMap(h)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
    pub fn get(&self, key: &str) -> Result<Option<&'de str>> {
        match self.0.get(key) {
            None => Ok(None),
            Some(v) => {
                Ok(Some(from_utf8(v.as_bytes())?))
            }
        }
    }
}

pub struct HeaderMapOwned(http::HeaderMap);

impl HeaderMapOwned {
    pub fn new(h: http::HeaderMap) -> Self {
        HeaderMapOwned(h)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn insert(&mut self, key: &'static str, value: &str) -> Result<()> {
        self.0.insert(key, value.parse()?);

        Ok(())
    }
}
