use crate::object::*;

impl Entry {
    pub fn to_ast_str_pretty(&mut self) -> Result<String, ReamError> {
        let raw = serde_json::to_string_pretty(&self).unwrap();
        Ok(raw)
    }

    pub fn to_ast_str(&mut self) -> Result<String, ReamError> {
        let raw = serde_json::to_string(&self).unwrap();
        Ok(raw)
    }
}
