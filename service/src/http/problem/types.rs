use super::{Problem, ProblemType, ProblemTypeStatus};
use actix_http::http::StatusCode;
use std::fmt::{Display, Formatter};

/// A simple representation of a problem type.
#[derive(Debug)]
pub struct SimpleProblemType {
    /// The actual problem code
    pub problem_type: &'static str,
    /// The title of the problem
    pub problem_title: &'static str,
    /// The default status code for the problem
    pub status_code: StatusCode,
}

impl ProblemType for SimpleProblemType {
    /// A URI Reference that identifies the problem type.
    fn problem_type(&self) -> &'static str {
        self.problem_type
    }
}

impl ProblemTypeStatus for SimpleProblemType {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl Display for SimpleProblemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.problem_title)
    }
}

impl From<SimpleProblemType> for Problem {
    fn from(problem_type: SimpleProblemType) -> Self {
        Self::new(problem_type)
    }
}

pub const NOT_FOUND: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:universe/2020:problems/not_found",
    problem_title: "The requested resource was not found",
    status_code: StatusCode::NOT_FOUND,
};
