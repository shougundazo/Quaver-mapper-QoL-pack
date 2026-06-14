use crate::model::{QuaMap, TimingPoint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResnapOptions {
    pub snap_divisor: i32,
    pub max_offset_ms: i32,
    pub include_long_note_ends: bool,
}

impl Default for ResnapOptions {
    fn default() -> Self {
        Self {
            snap_divisor: 4,
            max_offset_ms: 6,
            include_long_note_ends: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResnapMove {
    pub lane: i32,
    pub old_time: i32,
    pub new_time: i32,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResnapReport {
    pub moved_objects: usize,
    pub skipped_objects: usize,
    pub moves: Vec<ResnapMove>,
}

pub fn resnap_map(map: &mut QuaMap, options: &ResnapOptions) -> ResnapReport {
    let mut report = ResnapReport {
        moved_objects: 0,
        skipped_objects: 0,
        moves: Vec::new(),
    };

    if options.snap_divisor <= 0 || map.timing_points.is_empty() {
        report.skipped_objects = map.hit_objects.len();
        return report;
    }

    let timing_points = map.timing_points.clone();

    for object in &mut map.hit_objects {
        let start = object.start_time;
        if let Some(new_time) = nearest_allowed_snap(start, &timing_points, options) {
            if new_time != start {
                object.start_time = new_time;
                report.moved_objects += 1;
                report.moves.push(ResnapMove {
                    lane: object.lane,
                    old_time: start,
                    new_time,
                    kind: "start".to_string(),
                });
            }
        } else {
            report.skipped_objects += 1;
        }

        if options.include_long_note_ends && object.is_long_note() {
            let end = object.effective_end_time();
            if let Some(new_time) = nearest_allowed_snap(end, &timing_points, options) {
                if new_time != end && new_time > object.start_time {
                    object.end_time = Some(new_time);
                    report.moves.push(ResnapMove {
                        lane: object.lane,
                        old_time: end,
                        new_time,
                        kind: "end".to_string(),
                    });
                }
            }
        }
    }

    report
}

fn nearest_allowed_snap(
    time: i32,
    timing_points: &[TimingPoint],
    options: &ResnapOptions,
) -> Option<i32> {
    let timing = timing_points
        .iter()
        .rev()
        .find(|tp| tp.start_time <= time as f32)
        .or_else(|| timing_points.first())?;

    if timing.bpm <= 0.0 {
        return None;
    }

    let beat_ms = 60000.0 / timing.bpm;
    let grid_ms = beat_ms / options.snap_divisor as f32;
    if grid_ms <= 0.0 {
        return None;
    }

    let relative = time as f32 - timing.start_time;
    let snapped = timing.start_time + (relative / grid_ms).round() * grid_ms;
    let snapped_i32 = snapped.round() as i32;
    let offset = (snapped_i32 - time).abs();

    if offset <= options.max_offset_ms {
        Some(snapped_i32)
    } else {
        None
    }
}
