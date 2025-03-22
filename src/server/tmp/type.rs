use crate::*;

pub type OnceCellTmp = OnceCell<Tmp>;

#[derive(Clone, Lombok)]
pub struct Tmp {
    pub(super) log: Log,
}
