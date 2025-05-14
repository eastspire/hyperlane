use crate::*;

#[derive(Debug, Clone)]
pub struct RoutePattern(pub(super) VecRouteSegment);

#[derive(Clone)]
pub struct RouteMatcher<T>(pub(super) VecRoutePatternArcFunc<T>);
