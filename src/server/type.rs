use crate::*;

#[derive(Lombok, Default)]
pub struct Server {
    cfg: OnceCellServerConfig,
    route_func: ArcDashMapRouteFuncBox,
    request_middleware: OnceCellVecBoxFunc,
    response_middleware: OnceCellVecBoxFunc,
    tmp: OnceCellTmp,
}
