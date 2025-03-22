use crate::*;

pub type DashMapRouteFuncBox = DashMap<&'static str, BoxFunc>;
pub type ArcDashMapRouteFuncBox = Arc<DashMapRouteFuncBox>;
