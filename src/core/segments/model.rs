use super::{Segment, SegmentData};
use crate::config::{QwenInput, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelSegment;

impl ModelSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for ModelSegment {
    fn collect(&self, input: &QwenInput) -> Option<SegmentData> {
        let display_name = &input.model.display_name;
        if display_name.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        metadata.insert("display_name".to_string(), display_name.clone());

        Some(SegmentData {
            primary: display_name.clone(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Model
    }
}
