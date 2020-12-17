use super::{Problem, SimpleProblemType};
use actix_http::http::StatusCode;
use std::collections::HashMap;

const VALIDATION: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:universe/2020:problems/validation_error",
    problem_title: "The incoming request was not valid",
    status_code: StatusCode::UNPROCESSABLE_ENTITY,
};

/// Problem code for a required but missing field.
pub const VALIDATION_PROBLEM_MISSING_FIELD: &str = "tag:universe/2020:validations/missing_field";

/// RFC-7807 Problem builder for a Validation problem
#[derive(Debug, Default)]
pub struct ValidationProblem {
    fields: HashMap<String, String>,
}

impl ValidationProblem {
    /// Add a new field to the validation problem
    ///
    /// # Parameters
    /// - `field` - The field identifier
    /// - `problem` - The problem with the field
    pub fn with_field<F, P>(mut self, field: F, problem: P) -> Self
    where
        F: Into<String>,
        P: Into<String>,
    {
        self.fields.insert(field.into(), problem.into());
        self
    }

    /// Build an RFC-7807 problem from this builder
    ///
    /// # Returns
    /// The `Problem` instance
    pub fn build(self) -> Problem {
        Problem::new(VALIDATION).with_extra("fields", self.fields)
    }
}
