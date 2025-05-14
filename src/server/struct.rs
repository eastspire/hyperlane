use super::*;

#[derive(Clone, Lombok)]
pub struct Server {
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) config: ArcRwLockServerConfig<'static>,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) route: ArcRwLockHashMapRouteFuncBox<Context>,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) route_matcher: ArcRwLockRouteMatcher<Context>,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) request_middleware: ArcRwLockMiddlewareFuncBox<Context>,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) response_middleware: ArcRwLockMiddlewareFuncBox<Context>,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) tmp: ArcRwLockTmp,
}

#[derive(Clone)]
pub(crate) struct RequestHandlerImmutableParams<'a> {
    pub(super) stream: &'a ArcRwLockStream,
    pub(super) log: &'a Log,
    pub(super) buffer_size: usize,
    pub(super) request_middleware: &'a ArcRwLockMiddlewareFuncBox<Context>,
    pub(super) response_middleware: &'a ArcRwLockMiddlewareFuncBox<Context>,
    pub(super) route_func: &'a ArcRwLockHashMapRouteFuncBox<Context>,
    pub(super) route_matcher: &'a ArcRwLockRouteMatcher<Context>,
}
