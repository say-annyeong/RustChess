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
    pub fn from_str(json: &str) -> Result<FileFormat> {
        serde_json::from_str(json)
    }

    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_short_name(&self) -> &String {
        &self.short_name
    }

    pub fn get_point(&self) -> &i32 {
        &self.point
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }
}