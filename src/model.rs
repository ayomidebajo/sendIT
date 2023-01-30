use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]

pub enum FileType {
    TXT,
    ZIP,
    JPEG,
    PNG,
    PDF,
    ODT,
}

pub struct File<'a> {
    pub id: Option<usize>,
    pub file_type: FileType,
    pub file: &'a mut [u8],
    pub file_name: String,
}