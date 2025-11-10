use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct FileFormat {
    name: String,
    short_name: String,
    point: i32,
    code: String,
}

impl FileFormat {
    #[inline(always)]
    pub fn from_str(json: &str) -> Result<FileFormat> {
        serde_json::from_str(json)
    }

    #[inline(always)]
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    #[inline(always)]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    #[inline(always)]
    pub fn get_short_name(&self) -> &String {
        &self.short_name
    }

    #[inline(always)]
    pub fn get_point(&self) -> &i32 {
        &self.point
    }

    #[inline(always)]
    pub fn get_code(&self) -> &String {
        &self.code
    }
}