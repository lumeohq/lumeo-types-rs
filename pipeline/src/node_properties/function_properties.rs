use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FunctionProperties {
    pub module: String,
}
