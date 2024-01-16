/// Errors relating to `Params` specifications.
#[derive(Debug, PartialEq)]
pub enum ParameterSetError {
    InvalidOrder(String),
    ElementCount(String),
}
