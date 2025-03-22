use crate::*;

pub type WriteInnerControllerData<'a> = RwLockWriteGuard<'a, InnerControllerData>;
pub type ReadInnerControllerData<'a> = RwLockReadGuard<'a, InnerControllerData>;

pub type OptionWriteInnerControllerData<'a> = Option<WriteInnerControllerData<'a>>;
pub type OptionReadInnerControllerData<'a> = Option<ReadInnerControllerData<'a>>;
pub type OptionInnerControllerData = Option<InnerControllerData>;

#[derive(Clone, Debug, Lombok, Default)]
pub struct InnerControllerData {
    #[get_mut(skip)]
    stream_opt: OptionArcRwLockStream,
    #[get_mut(skip)]
    request: ArcRwLock<Request>,
    #[get_mut(skip)]
    response: ArcRwLock<Response>,
    #[get_mut(skip)]
    log: ArcRwLock<Log>,
}

#[derive(Clone, Debug)]
pub struct ControllerData(pub(super) Arc<InnerControllerData>);
