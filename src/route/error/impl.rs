use super::*;

impl StdError for RouteError {}

impl Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePattern(pattern) => {
                write!(f, "Route pattern already exists: {}", pattern)
            }
        }
    }
}
