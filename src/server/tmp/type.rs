use crate::*;

pub type RwLockTmp = RwLock<Tmp>;

#[derive(Clone, Lombok)]
pub struct Tmp {
    pub(super) log: Log,
}
