use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiffSummary {
    pub added_lines: usize,
    pub removed_lines: usize,
    pub changed: bool,
    pub unified: String,
}

pub fn diff_text(before: &str, after: &str) -> DiffSummary {
    let diff = TextDiff::from_lines(before, after);
    let mut added_lines = 0;
    let mut removed_lines = 0;

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => removed_lines += 1,
            ChangeTag::Insert => added_lines += 1,
            ChangeTag::Equal => {}
        }
    }

    let unified = diff
        .unified_diff()
        .context_radius(3)
        .header("before.qua", "after.qua")
        .to_string();

    DiffSummary {
        added_lines,
        removed_lines,
        changed: added_lines > 0 || removed_lines > 0,
        unified,
    }
}
