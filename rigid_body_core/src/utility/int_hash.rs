use rustc_hash::FxHasher;
use std::{
    collections::{
	HashMap,
	HashSet,
    },
    hash::BuildHasherDefault,
};

pub type IntHasher = BuildHasherDefault<FxHasher>;
pub type IntSet<K> = HashSet::<K, IntHasher>;
pub type IntMap<K, V> = HashMap::<K, V, IntHasher>;
