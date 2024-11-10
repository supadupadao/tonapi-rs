use serde::Deserialize;

pub type TONAPIResult<R, E = TONAPIError> = Result<R, E>;

#[derive(Deserialize, Debug)]
pub enum TONAPIError {
    /// Error on prepare state of request
    PrepareRequestError(String),
    /// Error on sending state of request
    SendRequestError(String),
    /// Error on parsing response state of request
    ParseRequestError(String),
}
