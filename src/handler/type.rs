use crate::*;

pub type ArcFunc<T> = Arc<dyn Func<T>>;
pub type VecArcFunc<T> = Vec<ArcFunc<T>>;
pub type PinBoxFutureSend = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
