//! config.rs, part of Skarn.
//!
//! This module contains the definition of a flexible configuration object.

use typemap::{TypeMap, Assoc};

pub trait ConfigItem<T>: Assoc<T> {
    fn default() -> T;
}

pub struct Config {
    pub map: TypeMap
}

impl Config {
    pub fn get<K, V>(&self) -> V where K: ConfigItem<V> {
        match self.map.find::<K, V>() {
            Some(val) => val,
            None => K::default()
        }
    }

    pub fn set<K, V>(&mut self, val: V) where K: ConfigItem<V> {
        self.map.insert::<K, V>(val);
    }
}
