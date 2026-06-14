use crate::model::QuaMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MacroKind {
    ShiftTime,
    MirrorLanes,
    SelectDensityWindow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MacroOptions {
    pub kind: MacroKind,
    pub start_time: Option<i32>,
    pub end_time: Option<i32>,
    pub lane: Option<i32>,
    pub amount_ms: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MacroReport {
    pub affected_objects: usize,
    pub selected_indices: Vec<usize>,
}

pub fn apply_macro(map: &mut QuaMap, options: &MacroOptions) -> MacroReport {
    match options.kind {
        MacroKind::ShiftTime => shift_time(map, options),
        MacroKind::MirrorLanes => mirror_lanes(map, options),
        MacroKind::SelectDensityWindow => select_window(map, options),
    }
}

fn shift_time(map: &mut QuaMap, options: &MacroOptions) -> MacroReport {
    let amount = options.amount_ms.unwrap_or_default();
    let mut affected = 0;

    for object in &mut map.hit_objects {
        if matches_filter(object.start_time, object.lane, options) {
            object.start_time += amount;
            if let Some(end_time) = object.end_time.as_mut() {
                if *end_time > 0 {
                    *end_time += amount;
                }
            }
            affected += 1;
        }
    }

    MacroReport {
        affected_objects: affected,
        selected_indices: Vec::new(),
    }
}

fn mirror_lanes(map: &mut QuaMap, options: &MacroOptions) -> MacroReport {
    let Some(key_count) = map.key_count() else {
        return MacroReport {
            affected_objects: 0,
            selected_indices: Vec::new(),
        };
    };

    let mut affected = 0;
    for object in &mut map.hit_objects {
        if matches_filter(object.start_time, object.lane, options) {
            object.lane = key_count + 1 - object.lane;
            affected += 1;
        }
    }

    MacroReport {
        affected_objects: affected,
        selected_indices: Vec::new(),
    }
}

fn select_window(map: &QuaMap, options: &MacroOptions) -> MacroReport {
    let selected_indices = map
        .hit_objects
        .iter()
        .enumerate()
        .filter_map(|(index, object)| {
            matches_filter(object.start_time, object.lane, options).then_some(index)
        })
        .collect::<Vec<_>>();

    MacroReport {
        affected_objects: selected_indices.len(),
        selected_indices,
    }
}

fn matches_filter(time: i32, lane: i32, options: &MacroOptions) -> bool {
    if let Some(start) = options.start_time {
        if time < start {
            return false;
        }
    }

    if let Some(end) = options.end_time {
        if time > end {
            return false;
        }
    }

    if let Some(filter_lane) = options.lane {
        if lane != filter_lane {
            return false;
        }
    }

    true
}
