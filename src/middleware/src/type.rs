use super::*;

pub type ArcRwLockMiddlewareFuncBox<T> = ArcRwLock<VecArcFunc<T>>;
