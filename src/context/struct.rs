use super::*;

#[derive(Clone, Lombok, Default)]
pub struct InnerContext {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
    attribute: HashMapArcAnySendSync,
    route_params: ArcRwLockRouteParams,
    aborted: bool,
}

#[derive(Clone, Default)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
