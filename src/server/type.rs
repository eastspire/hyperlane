use crate::*;

#[derive(Lombok, Default)]
pub struct Server {
    cfg: RwLockServerConfig,
    route_func: ArcDashMapRouteFuncBox,
    request_middleware: ArcRwLockVecBoxFunc,
    response_middleware: ArcRwLockVecBoxFunc,
    tmp: RwLockTmp,
}
