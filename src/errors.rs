use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Error)]
#[error("Number of weights must be at least 1")]
pub struct NotEnoughWeightsError;