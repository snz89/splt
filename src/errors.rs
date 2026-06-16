use thiserror::Error;

#[derive(Debug, Error)]
#[error("Number of weights must be at least 1")]
pub struct NotEnoughWeightsError;

#[derive(Debug, Error)]
#[error(
    "terminal input is not supported: please pipe data into this command or provide an input argument"
)]
pub struct TerminalInputNotSupportedError;
