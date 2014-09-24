//! config.rs, part of Skarn.
//!
//! This module contains the definition of a flexible configuration object.

use typemap::{TypeMap, Assoc};
use phantom::Phantom;

use compare::{ComparisonMethodTrait, Content};

// XXX: Use infallible plugins from reem/rust-plugin once HKTs land.
pub trait ConfigItem<V>: Assoc<V> {
    fn default(Phantom<Self>) -> V;
}

pub struct Config {
    pub map: TypeMap
}

impl Config {
    pub fn new() -> Config {
        Config { map: TypeMap::new() }
    }

    pub fn get<K, V>(&mut self) -> &V where K: ConfigItem<V>, V: 'static {
        let found = self.map.contains::<K, V>();
        if found {
            return self.map.find::<K, V>().unwrap();
        }
        let default: V = ConfigItem::default(Phantom::<K>);
        self.set::<K, V>(default);
        self.get::<K, V>()
    }

    pub fn set<K, V>(&mut self, val: V) where K: ConfigItem<V>, V: 'static {
        self.map.insert::<K, V>(val);
    }
}

// Concrete config items.

pub type ComparisonMethod = Box<ComparisonMethodTrait + 'static>;

impl Assoc<ComparisonMethod> for ComparisonMethod {}

impl ConfigItem<ComparisonMethod> for ComparisonMethod {
    fn default(_: Phantom<ComparisonMethod>) -> ComparisonMethod {
        box Content as Box<ComparisonMethodTrait>
    }
}
