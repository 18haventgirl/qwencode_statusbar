use super::{Segment, SegmentData};
use crate::config::{QwenInput, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct DirectorySegment;

impl DirectorySegment {
    pub fn new() -> Self {
        Self
    }

    fn extract_directory_name(path: &str) -> String {
        let unix_name = path.split('/').next_back().unwrap_or("");
        let windows_name = path.split('\\').next_back().unwrap_or("");

        let result = if windows_name.len() < path.len() {
            windows_name
        } else if unix_name.len() < path.len() {
            unix_name
        } else {
            path
        };

        if result.is_empty() {
            "root".to_string()
        } else {
            result.to_string()
        }
    }
}

impl Segment for DirectorySegment {
    fn collect(&self, input: &QwenInput) -> Option<SegmentData> {
        let current_dir = &input.workspace.current_dir;
        if current_dir.is_empty() {
            return None;
        }

        let dir_name = Self::extract_directory_name(current_dir);

        let mut metadata = HashMap::new();
        metadata.insert("full_path".to_string(), current_dir.clone());

        Some(SegmentData {
            primary: dir_name,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Directory
    }
}
