use crate::*;

impl<F> Func for F where
    F: Fn(ControllerData) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
        + Send
        + Sync
        + 'static
{
}

impl<F, Fut> FuncWithoutPin<Fut> for F
where
    F: Fn(ControllerData) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
}
