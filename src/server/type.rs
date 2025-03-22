use crate::*;

#[derive(Lombok, Default)]
pub struct Server {
    cfg: RefCellServerConfig,
    route_func: ArcDashMapRouteFuncBox,
    request_middleware: RefCellVecBoxFunc,
    response_middleware: RefCellVecBoxFunc,
    tmp: RefCellTmp,
}
