//! config.rs, part of Skarn.
//!
//! This module contains the definition of a flexible configuration object.

use std::collections::HashSet;

use typemap::{TypeMap, Assoc};
use typemap::Entry::{Occupied, Vacant};
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
        match self.map.entry::<K, V>() {
            Occupied(e) => &*e.into_mut(),
            Vacant(e) => &*e.set(ConfigItem::default(Phantom::<K>))
        }
    }

    pub fn set<K, V>(&mut self, val: V) where K: ConfigItem<V>, V: 'static {
        self.map.insert::<K, V>(val);
    }
}

// Concrete config items
pub struct SourceDir;

impl Assoc<Path> for SourceDir {}

impl ConfigItem<Path> for SourceDir {
    fn default(_: Phantom<SourceDir>) -> Path {
        Path::new("")
    }
}

pub struct DestDir;

impl Assoc<Path> for DestDir {}

impl ConfigItem<Path> for DestDir {
    fn default(_: Phantom<DestDir>) -> Path {
        Path::new("")
    }
}

pub type ComparisonMethod = Box<ComparisonMethodTrait + 'static>;

impl Assoc<ComparisonMethod> for ComparisonMethod {}

impl ConfigItem<ComparisonMethod> for ComparisonMethod {
    fn default(_: Phantom<ComparisonMethod>) -> ComparisonMethod {
        box Content as Box<ComparisonMethodTrait>
    }
}


pub struct IncludeByDefault;

impl Assoc<bool> for IncludeByDefault {}

impl ConfigItem<bool> for IncludeByDefault {
    fn default(_: Phantom<IncludeByDefault>) -> bool {
        true
    }
}


#[deriving(PartialEq, Eq, Hash, Show, Clone)]
pub enum DeleteBehaviour {
    IncludedNoEquiv,
    ExcludedEquiv,
    ExcludedNoEquiv
}

impl Assoc<HashSet<DeleteBehaviour>> for DeleteBehaviour {}

impl ConfigItem<HashSet<DeleteBehaviour>> for DeleteBehaviour {
    fn default(_: Phantom<DeleteBehaviour>) -> HashSet<DeleteBehaviour> {
        HashSet::new()
    }
}
