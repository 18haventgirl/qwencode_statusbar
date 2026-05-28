pub mod context_window;
pub mod directory;
pub mod git;
pub mod metrics;
pub mod model;
pub mod session;

use crate::config::{QwenInput, SegmentId};
use std::collections::HashMap;

pub trait Segment {
    fn collect(&self, input: &QwenInput) -> Option<SegmentData>;
    fn id(&self) -> SegmentId;
}

#[derive(Debug, Clone)]
pub struct SegmentData {
    pub primary: String,
    pub secondary: String,
    pub metadata: HashMap<String, String>,
}

pub use context_window::ContextWindowSegment;
pub use directory::DirectorySegment;
pub use git::GitSegment;
pub use metrics::MetricsSegment;
pub use model::ModelSegment;
pub use session::SessionSegment;
