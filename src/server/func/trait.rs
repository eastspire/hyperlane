use crate::*;

pub trait Func:
    FnMut(&mut ControllerData) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
    + Send
    + Sync
    + 'static
{
}

pub trait FuncWithoutPin<Fut>: FnMut(&mut ControllerData) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}
