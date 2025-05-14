use super::*;

pub type VecRouteSegment = Vec<RouteSegment>;
pub type VecRoutePatternArcFunc<T> = Vec<(RoutePattern, ArcFunc<T>)>;
pub type ArcRwLockRouteMatcher<T> = ArcRwLock<RouteMatcher<T>>;
pub type HashMapRouteFuncBox<T> = HashMap<String, ArcFunc<T>, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockHashMapRouteFuncBox<T> = ArcRwLock<HashMapRouteFuncBox<T>>;
pub type TupleArcFuncRouteParams<T> = (ArcFunc<T>, RouteParams);
pub type OptionTupleArcFuncRouteParams<T> = Option<TupleArcFuncRouteParams<T>>;
pub type ResultAddRoute = Result<(), RouteError>;

pub type RouteParams = HashMap<String, String, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockRouteParams = ArcRwLock<RouteParams>;
