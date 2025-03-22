use crate::*;

pub type WriteInnerControllerData<'a> = RwLockWriteGuard<'a, InnerControllerData>;
pub type ReadInnerControllerData<'a> = RwLockReadGuard<'a, InnerControllerData>;

pub type OptionWriteInnerControllerData<'a> = Option<WriteInnerControllerData<'a>>;
pub type OptionReadInnerControllerData<'a> = Option<ReadInnerControllerData<'a>>;
pub type OptionInnerControllerData = Option<InnerControllerData>;

#[derive(Clone, Debug, Lombok, Default)]
pub struct InnerControllerData {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
}

#[derive(Clone, Debug)]
pub struct ControllerData(pub(super) ArcRwLock<InnerControllerData>);
