use crate::*;

impl<F, T> Func<T> for F
where
    T: Send + Sync,
    F: Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
{
}

impl<F, Fut, T> FuncWithoutPin<Fut, T> for F
where
    T: Send + Sync,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
{
}
