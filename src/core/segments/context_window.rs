use super::{Segment, SegmentData};
use crate::config::{QwenInput, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ContextWindowSegment;

impl ContextWindowSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for ContextWindowSegment {
    fn collect(&self, input: &QwenInput) -> Option<SegmentData> {
        let ctx = &input.context_window;

        if ctx.current_usage == 0 {
            return None;
        }

        let tokens_display = if ctx.current_usage >= 1000 {
            let k_value = ctx.current_usage as f64 / 1000.0;
            if k_value.fract() == 0.0 {
                format!("{}k", k_value as u32)
            } else {
                format!("{:.1}k", k_value)
            }
        } else {
            ctx.current_usage.to_string()
        };

        let mut metadata = HashMap::new();
        metadata.insert("tokens".to_string(), ctx.current_usage.to_string());
        metadata.insert("percentage".to_string(), ctx.used_percentage.to_string());
        metadata.insert("limit".to_string(), ctx.context_window_size.to_string());
        metadata.insert(
            "total_input".to_string(),
            ctx.total_input_tokens.to_string(),
        );
        metadata.insert(
            "total_output".to_string(),
            ctx.total_output_tokens.to_string(),
        );

        Some(SegmentData {
            primary: format!("{} tokens", tokens_display),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::ContextWindow
    }
}
