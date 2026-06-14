use crate::model::{HitObject, QuaMap};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CheckIssue {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub start_time: Option<i32>,
    pub lane: Option<i32>,
}

pub fn check_map(map: &QuaMap) -> Vec<CheckIssue> {
    let mut issues = Vec::new();

    if map.timing_points.is_empty() {
        issues.push(issue(
            Severity::Error,
            "missing_timing_points",
            "Map has no TimingPoints.",
            None,
            None,
        ));
    }

    if map.hit_objects.is_empty() {
        issues.push(issue(
            Severity::Warning,
            "missing_hit_objects",
            "Map has no HitObjects.",
            None,
            None,
        ));
    }

    if map.mode.is_none() {
        issues.push(issue(
            Severity::Warning,
            "missing_mode",
            "Map Mode is missing.",
            None,
            None,
        ));
    }

    let key_count = map.key_count();
    let mut seen_objects = HashSet::new();
    let timing_group_ids: HashSet<&str> = map.timing_groups.keys().map(String::as_str).collect();

    for object in &map.hit_objects {
        check_object(
            map,
            object,
            key_count,
            &timing_group_ids,
            &mut seen_objects,
            &mut issues,
        );
    }

    for pair in map.hit_objects.windows(2) {
        if pair[0].start_time > pair[1].start_time {
            issues.push(issue(
                Severity::Info,
                "unsorted_hit_objects",
                "HitObjects are not sorted by StartTime; saving will normalize order.",
                Some(pair[1].start_time),
                Some(pair[1].lane),
            ));
            break;
        }
    }

    let mut lanes_by_time: BTreeMap<i32, HashSet<i32>> = BTreeMap::new();
    for object in &map.hit_objects {
        if !lanes_by_time
            .entry(object.start_time)
            .or_default()
            .insert(object.lane)
        {
            issues.push(issue(
                Severity::Warning,
                "stacked_note",
                "Multiple HitObjects share the same StartTime and Lane.",
                Some(object.start_time),
                Some(object.lane),
            ));
        }
    }

    for timing in &map.timing_points {
        if timing.bpm <= 0.0 {
            issues.push(issue(
                Severity::Error,
                "invalid_bpm",
                "TimingPoint BPM must be greater than zero.",
                Some(timing.start_time.round() as i32),
                None,
            ));
        }
    }

    issues
}

fn check_object(
    map: &QuaMap,
    object: &HitObject,
    key_count: Option<i32>,
    timing_group_ids: &HashSet<&str>,
    seen_objects: &mut HashSet<(i32, i32, i32)>,
    issues: &mut Vec<CheckIssue>,
) {
    if object.start_time < 0 {
        issues.push(issue(
            Severity::Warning,
            "negative_start_time",
            "HitObject StartTime is negative.",
            Some(object.start_time),
            Some(object.lane),
        ));
    }

    if let Some(keys) = key_count {
        if object.lane < 1 || object.lane > keys {
            issues.push(issue(
                Severity::Error,
                "lane_out_of_range",
                &format!("HitObject Lane is outside 1..={keys}."),
                Some(object.start_time),
                Some(object.lane),
            ));
        }
    }

    if object.is_long_note() && object.effective_end_time() <= object.start_time {
        issues.push(issue(
            Severity::Error,
            "invalid_long_note",
            "Long note EndTime must be greater than StartTime.",
            Some(object.start_time),
            Some(object.lane),
        ));
    }

    if !object.timing_group.is_empty()
        && object.timing_group != "$Default"
        && !timing_group_ids.contains(object.timing_group.as_str())
    {
        issues.push(issue(
            Severity::Warning,
            "missing_timing_group",
            "HitObject references a TimingGroup that is not defined.",
            Some(object.start_time),
            Some(object.lane),
        ));
    }

    for key_sound in &object.key_sounds {
        if key_sound.sample < 1 || key_sound.sample > map.custom_audio_samples.len() as i32 {
            issues.push(issue(
                Severity::Warning,
                "invalid_keysound_sample",
                "KeySound Sample index does not exist in CustomAudioSamples.",
                Some(object.start_time),
                Some(object.lane),
            ));
        }
    }

    let identity = (object.start_time, object.lane, object.effective_end_time());
    if !seen_objects.insert(identity) {
        issues.push(issue(
            Severity::Warning,
            "duplicate_hit_object",
            "Duplicate HitObject detected.",
            Some(object.start_time),
            Some(object.lane),
        ));
    }
}

fn issue(
    severity: Severity,
    code: &str,
    message: &str,
    start_time: Option<i32>,
    lane: Option<i32>,
) -> CheckIssue {
    CheckIssue {
        severity,
        code: code.to_string(),
        message: message.to_string(),
        start_time,
        lane,
    }
}
