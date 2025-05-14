#[derive(Debug, Clone)]
pub enum RouteSegment {
    Static(String),
    Dynamic(String),
}
