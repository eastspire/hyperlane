use crate::*;

pub trait Func<T>: Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync {}

pub trait FuncWithoutPin<Fut, T>: Fn(T) -> Fut + Send + Sync + 'static
where
    T: Send + Sync,
    Fut: Future<Output = ()> + Send,
{
}
