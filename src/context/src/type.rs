use crate::*;

pub type RwLockWriteInnerContext<'a> = RwLockWriteGuard<'a, InnerContext>;
pub type RwLockReadInnerContext<'a> = RwLockReadGuard<'a, InnerContext>;
pub type HashMapArcAnySendSync = HashMap<String, ArcAnySendSync>;
