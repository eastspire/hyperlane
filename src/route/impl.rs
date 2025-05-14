use super::*;

impl PartialEq for RouteSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RouteSegment::Static(segment1), RouteSegment::Static(segment2)) => {
                segment1 == segment2
            }
            (RouteSegment::Dynamic(_), RouteSegment::Dynamic(_)) => true,
            _ => false,
        }
    }
}

impl PartialEq for RoutePattern {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(segment1, segment2)| segment1 == segment2)
    }
}

impl RoutePattern {
    pub fn new(route: &str) -> Self {
        let segments: VecRouteSegment = Self::parse_route(route);
        Self(segments)
    }

    fn parse_route(route: &str) -> VecRouteSegment {
        let mut segments: VecRouteSegment = Vec::new();
        let route: &str = route.trim_start_matches(DEFAULT_HTTP_PATH);
        if route.is_empty() {
            return segments;
        }
        for segment in route.split(DEFAULT_HTTP_PATH) {
            if segment.starts_with(COLON_SPACE_SYMBOL) {
                let param_name: String = segment[1..].to_string();
                segments.push(RouteSegment::Dynamic(param_name));
            } else {
                segments.push(RouteSegment::Static(segment.to_string()));
            }
        }
        segments
    }

    pub fn match_path(&self, path: &str) -> Option<RouteParams> {
        let path: &str = path.trim_start_matches(DEFAULT_HTTP_PATH);
        let path_segments: Vec<&str> = if path.is_empty() {
            Vec::new()
        } else {
            path.split(DEFAULT_HTTP_PATH).collect()
        };
        if path_segments.len() != self.0.len() {
            return None;
        }
        let mut params: RouteParams = hash_map_xx_hash3_64();
        for (idx, segment) in self.0.iter().enumerate() {
            match segment {
                RouteSegment::Static(path) => {
                    if path != path_segments[idx] {
                        return None;
                    }
                }
                RouteSegment::Dynamic(param_name) => {
                    params.insert(param_name.clone(), path_segments[idx].to_string());
                }
            }
        }
        Some(params)
    }
}

impl RouteMatcher {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, pattern: &str, handler: ArcFunc) -> ResultAddRoute {
        let route_pattern: RoutePattern = RoutePattern::new(pattern);
        let has_same_pattern: bool = self
            .0
            .iter()
            .any(|(tmp_pattern, _)| tmp_pattern == &route_pattern);
        if has_same_pattern {
            return Err(RouteError::DuplicatePattern(pattern.to_string()));
        }
        self.0.push((route_pattern, handler));
        return Ok(());
    }

    pub fn match_route(&self, path: &str) -> OptionTupleArcFuncRouteParams {
        for (pattern, handler) in &self.0 {
            if let Some(params) = pattern.match_path(path) {
                return Some((handler.clone(), params));
            }
        }
        None
    }
}
