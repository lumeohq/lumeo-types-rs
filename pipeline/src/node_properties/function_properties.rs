use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionProperties {
    pub code: String,
    #[serde(flatten)]
    pub runtime: FunctionRuntime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionRuntime {
    /// Path to python module.
    ///
    /// This field is set to `Some` by `lumeod`.
    ///
    /// `lumeod` receives code as a string blob (`FunctionProperties::code`) but gvapython
    /// element requires a path to ".py" file, so lumeod stores code in a file and sets
    /// `module` field to file path.
    pub module: Option<std::path::PathBuf>,
}
