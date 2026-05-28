use super::{Segment, SegmentData};
use crate::config::{QwenInput, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct SessionSegment;

impl SessionSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for SessionSegment {
    fn collect(&self, input: &QwenInput) -> Option<SegmentData> {
        let files = &input.metrics.files;
        let has_changes = files.total_lines_added > 0 || files.total_lines_removed > 0;

        if !has_changes {
            return None;
        }

        let primary = format!(
            "\x1b[32m+{}\x1b[0m \x1b[31m-{}\x1b[0m",
            files.total_lines_added, files.total_lines_removed
        );

        let mut metadata = HashMap::new();
        metadata.insert(
            "lines_added".to_string(),
            files.total_lines_added.to_string(),
        );
        metadata.insert(
            "lines_removed".to_string(),
            files.total_lines_removed.to_string(),
        );

        Some(SegmentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Session
    }
}
