use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_zero_i32(value: &i32) -> bool {
    *value == 0
}

fn is_zero_f32(value: &f32) -> bool {
    *value == 0.0
}

fn is_empty_string(value: &String) -> bool {
    value.is_empty()
}

fn default_timing_group() -> String {
    "$Default".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QuaMap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qua_version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub song_preview_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_set_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub difficulty_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub legacy_ln_rendering: bool,
    #[serde(
        default,
        rename = "BPMDoesNotAffectScrollVelocity",
        alias = "BpmDoesNotAffectScrollVelocity",
        skip_serializing_if = "is_false"
    )]
    pub bpm_does_not_affect_scroll_velocity: bool,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub initial_scroll_velocity: f32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_scratch_key: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor_layers: Vec<EditorLayer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bookmarks: Vec<Bookmark>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_audio_samples: Vec<CustomAudioSample>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sound_effects: Vec<SoundEffect>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing_points: Vec<TimingPoint>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slider_velocities: Vec<SliderVelocity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scroll_speed_factors: Vec<ScrollSpeedFactor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hit_objects: Vec<HitObject>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub timing_groups: BTreeMap<String, TimingGroup>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl QuaMap {
    pub fn sort_for_write(&mut self) {
        self.bookmarks.sort_by_key(|b| b.start_time);
        self.hit_objects
            .sort_by_key(|h| (h.start_time, h.lane, h.end_time.unwrap_or_default()));
        self.sound_effects.sort_by_key(|s| s.start_time);
        self.timing_points
            .sort_by(|a, b| a.start_time.total_cmp(&b.start_time));
        self.slider_velocities
            .sort_by(|a, b| a.start_time.total_cmp(&b.start_time));
        self.scroll_speed_factors
            .sort_by(|a, b| a.start_time.total_cmp(&b.start_time));

        for group in self.timing_groups.values_mut() {
            group
                .scroll_velocities
                .sort_by(|a, b| a.start_time.total_cmp(&b.start_time));
            group
                .scroll_speed_factors
                .sort_by(|a, b| a.start_time.total_cmp(&b.start_time));
        }
    }

    pub fn key_count(&self) -> Option<i32> {
        let mode = self.mode.as_deref()?.to_ascii_lowercase();
        let base = match mode.as_str() {
            "keys4" | "4k" => 4,
            "keys7" | "7k" => 7,
            _ => return None,
        };
        Some(base + i32::from(self.has_scratch_key))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TimingPoint {
    #[serde(default)]
    pub start_time: f32,
    #[serde(default)]
    pub bpm: f32,
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub signature: i32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub hidden: bool,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SliderVelocity {
    #[serde(default)]
    pub start_time: f32,
    #[serde(default)]
    pub multiplier: f32,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ScrollSpeedFactor {
    #[serde(default)]
    pub start_time: f32,
    #[serde(default)]
    pub multiplier: f32,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct HitObject {
    #[serde(default)]
    pub start_time: i32,
    #[serde(default)]
    pub lane: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_sound: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub key_sounds: Vec<KeySound>,
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub editor_layer: i32,
    #[serde(
        default = "default_timing_group",
        skip_serializing_if = "is_default_timing_group"
    )]
    pub timing_group: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl HitObject {
    pub fn effective_end_time(&self) -> i32 {
        self.end_time.unwrap_or(0)
    }

    pub fn is_long_note(&self) -> bool {
        self.effective_end_time() > 0
    }
}

fn is_default_timing_group(value: &String) -> bool {
    value == "$Default" || value.is_empty()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct KeySound {
    #[serde(default)]
    pub sample: i32,
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub volume: i32,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Bookmark {
    #[serde(default)]
    pub start_time: i32,
    #[serde(default, skip_serializing_if = "is_empty_string")]
    pub note: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkExtension {
    pub start_time: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub orphan: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TimingGroup {
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub initial_scroll_velocity: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scroll_velocities: Vec<SliderVelocity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scroll_speed_factors: Vec<ScrollSpeedFactor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct EditorLayer {
    #[serde(default, skip_serializing_if = "is_empty_string")]
    pub name: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub hidden: bool,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CustomAudioSample {
    #[serde(default, skip_serializing_if = "is_empty_string")]
    pub path: String,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SoundEffect {
    #[serde(default)]
    pub start_time: i32,
    #[serde(default)]
    pub sample: i32,
    #[serde(default, skip_serializing_if = "is_zero_i32")]
    pub volume: i32,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
