use quaver_qol_core::{
    check_map, create_backup, list_backups, merge_bookmark_extensions, parse_qua_str, resnap_map,
    write_qua, BookmarkExtension, ResnapOptions, Severity, WriteOptions,
};
use std::fs;

const SAMPLE: &str = r#"
Mode: Keys4
Title: Test Song
Artist: Tester
DifficultyName: MVP
TimingPoints:
  - StartTime: 0
    Bpm: 120
HitObjects:
  - StartTime: 501
    Lane: 1
  - StartTime: 750
    Lane: 5
  - StartTime: 1000
    Lane: 2
    EndTime: 900
Bookmarks:
  - StartTime: 500
    Note: Verse
"#;

#[test]
fn parses_qua_model() {
    let map = parse_qua_str(SAMPLE).unwrap();
    assert_eq!(map.mode.as_deref(), Some("Keys4"));
    assert_eq!(map.hit_objects.len(), 3);
    assert_eq!(map.bookmarks[0].note, "Verse");
}

#[test]
fn checker_finds_basic_errors() {
    let map = parse_qua_str(SAMPLE).unwrap();
    let issues = check_map(&map);
    assert!(issues
        .iter()
        .any(|i| i.severity == Severity::Error && i.code == "lane_out_of_range"));
    assert!(issues
        .iter()
        .any(|i| i.severity == Severity::Error && i.code == "invalid_long_note"));
}

#[test]
fn resnap_moves_near_grid_notes() {
    let mut map = parse_qua_str(SAMPLE).unwrap();
    let report = resnap_map(
        &mut map,
        &ResnapOptions {
            snap_divisor: 4,
            max_offset_ms: 6,
            include_long_note_ends: false,
        },
    );
    assert_eq!(report.moved_objects, 1);
    assert_eq!(map.hit_objects[0].start_time, 500);
}

#[test]
fn write_creates_backup_when_not_dry_run() {
    let dir = tempfile::tempdir().unwrap();
    let qua_path = dir.path().join("map.qua");
    fs::write(&qua_path, SAMPLE).unwrap();
    let mut map = parse_qua_str(SAMPLE).unwrap();
    map.title = Some("Changed".to_string());

    let diff = write_qua(
        &qua_path,
        &map,
        &WriteOptions {
            dry_run: false,
            backup_dir: None,
        },
    )
    .unwrap();

    assert!(diff.changed);
    let backups = fs::read_dir(dir.path().join(".quaver-qol-backups"))
        .unwrap()
        .count();
    assert_eq!(backups, 1);
}

#[test]
fn dry_run_does_not_write_or_backup() {
    let dir = tempfile::tempdir().unwrap();
    let qua_path = dir.path().join("map.qua");
    fs::write(&qua_path, SAMPLE).unwrap();
    let mut map = parse_qua_str(SAMPLE).unwrap();
    map.title = Some("Dry Run".to_string());

    let diff = write_qua(&qua_path, &map, &WriteOptions::default()).unwrap();

    assert!(diff.changed);
    assert!(fs::read_to_string(&qua_path).unwrap().contains("Test Song"));
    assert!(!dir.path().join(".quaver-qol-backups").exists());
}

#[test]
fn sidecar_marks_orphans_without_creating_conflicts() {
    let map = parse_qua_str(SAMPLE).unwrap();
    let merged = merge_bookmark_extensions(
        &map,
        &[
            BookmarkExtension {
                start_time: 500,
                label: Some("Verse".to_string()),
                ..Default::default()
            },
            BookmarkExtension {
                start_time: 9999,
                label: Some("Lost".to_string()),
                ..Default::default()
            },
        ],
    );

    assert!(!merged[0].orphan);
    assert!(merged[1].orphan);
}

#[test]
fn can_create_explicit_backup() {
    let dir = tempfile::tempdir().unwrap();
    let qua_path = dir.path().join("map.qua");
    fs::write(&qua_path, SAMPLE).unwrap();

    let entry = create_backup(&qua_path, None).unwrap();
    assert!(entry.backup_path.exists());
}

#[test]
fn lists_backup_history() {
    let dir = tempfile::tempdir().unwrap();
    let qua_path = dir.path().join("map.qua");
    fs::write(&qua_path, SAMPLE).unwrap();

    create_backup(&qua_path, None).unwrap();
    let backups = list_backups(&qua_path).unwrap();

    assert_eq!(backups.len(), 1);
    assert!(backups[0].file_name.ends_with(".qua.bak"));
}

#[test]
fn accepts_quaver_bpm_uppercase_field() {
    let map = parse_qua_str(
        r#"
Mode: Keys4
BPMDoesNotAffectScrollVelocity: true
TimingPoints:
  - StartTime: 0
    Bpm: 180
"#,
    )
    .unwrap();

    assert!(map.bpm_does_not_affect_scroll_velocity);
}
