use crate::*;

pub type RefMutInnerControllerData<'a> = &'a mut InnerControllerData;
pub type RefInnerControllerData<'a> = &'a InnerControllerData;

pub type OptionRefMutInnerControllerData<'a> = Option<RefMutInnerControllerData<'a>>;
pub type OptionRefInnerControllerData<'a> = Option<RefInnerControllerData<'a>>;
pub type OptionInnerControllerData = Option<InnerControllerData>;

#[derive(Clone, Debug, Lombok, Default)]
pub struct InnerControllerData {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
}

#[derive(Clone, Debug)]
pub struct ControllerData(pub(super) OnceCell<InnerControllerData>);
