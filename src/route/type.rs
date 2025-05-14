use super::*;

pub type VecRouteSegment = Vec<RouteSegment>;
pub type VecRoutePatternArcFunc = Vec<(RoutePattern, ArcFunc)>;
pub type ArcRwLockRouteMatcher = ArcRwLock<RouteMatcher>;
pub type HashMapRouteFuncBox = HashMap<String, ArcFunc, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockHashMapRouteFuncBox = ArcRwLock<HashMapRouteFuncBox>;
pub type TupleArcFuncRouteParams = (ArcFunc, RouteParams);
pub type OptionTupleArcFuncRouteParams = Option<TupleArcFuncRouteParams>;
pub type ResultAddRoute = Result<(), RouteError>;

pub type RouteParams = HashMap<String, String, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockRouteParams = ArcRwLock<RouteParams>;
