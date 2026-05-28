use super::{Segment, SegmentData};
use crate::config::{QwenInput, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct MetricsSegment;

impl MetricsSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for MetricsSegment {
    fn collect(&self, input: &QwenInput) -> Option<SegmentData> {
        let metrics = &input.metrics;

        if metrics.models.is_empty() {
            return None;
        }

        let mut total_requests = 0u32;
        let mut total_errors = 0u32;
        let mut total_latency_ms = 0u64;
        let mut total_prompt = 0u32;
        let mut total_completion = 0u32;
        let mut total_cached = 0u32;
        let mut total_thoughts = 0u32;

        for model_metrics in metrics.models.values() {
            total_requests += model_metrics.api.total_requests;
            total_errors += model_metrics.api.total_errors;
            total_latency_ms += model_metrics.api.total_latency_ms;
            total_prompt += model_metrics.tokens.prompt;
            total_completion += model_metrics.tokens.completion;
            total_cached += model_metrics.tokens.cached;
            total_thoughts += model_metrics.tokens.thoughts;
        }

        let avg_latency = if total_requests > 0 {
            total_latency_ms / total_requests as u64
        } else {
            0
        };

        let primary = format!("{} req", total_requests);

        let mut secondary_parts = Vec::new();
        if avg_latency > 0 {
            secondary_parts.push(format!("{}ms avg", avg_latency));
        }
        if total_errors > 0 {
            secondary_parts.push(format!("\x1b[31m{} err\x1b[0m", total_errors));
        }

        let mut metadata = HashMap::new();
        metadata.insert("total_requests".to_string(), total_requests.to_string());
        metadata.insert("total_errors".to_string(), total_errors.to_string());
        metadata.insert("avg_latency_ms".to_string(), avg_latency.to_string());
        metadata.insert("total_prompt".to_string(), total_prompt.to_string());
        metadata.insert(
            "total_completion".to_string(),
            total_completion.to_string(),
        );
        metadata.insert("total_cached".to_string(), total_cached.to_string());
        metadata.insert("total_thoughts".to_string(), total_thoughts.to_string());

        Some(SegmentData {
            primary,
            secondary: secondary_parts.join(" · "),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Metrics
    }
}
