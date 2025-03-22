use crate::*;

pub type RefCellTmp = RefCell<Tmp>;

#[derive(Clone, Lombok)]
pub struct Tmp {
    pub(super) log: Log,
}
