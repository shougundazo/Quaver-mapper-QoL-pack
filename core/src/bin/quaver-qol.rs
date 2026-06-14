use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use quaver_qol_core::{
    apply_macro, check_map, create_backup, diff_text, load_bookmark_sidecar, load_qua,
    merge_bookmark_extensions, resnap_map, restore_backup, save_bookmark_sidecar, write_qua,
    MacroKind, MacroOptions, ResnapOptions, WriteOptions,
};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "quaver-qol")]
#[command(about = "Quaver Mapper QoL Pack CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Check {
        qua: PathBuf,
    },
    Diff {
        qua: PathBuf,
    },
    Resnap {
        qua: PathBuf,
        #[arg(long, default_value_t = 4)]
        snap: i32,
        #[arg(long, default_value_t = 6)]
        max_offset_ms: i32,
        #[arg(long, default_value_t = true)]
        include_long_note_ends: bool,
        #[arg(long)]
        write: bool,
    },
    Backup {
        qua: PathBuf,
        #[arg(long)]
        backup_dir: Option<PathBuf>,
    },
    Restore {
        backup: PathBuf,
        qua: PathBuf,
    },
    Macro {
        qua: PathBuf,
        #[arg(long, value_enum)]
        kind: CliMacroKind,
        #[arg(long)]
        start_time: Option<i32>,
        #[arg(long)]
        end_time: Option<i32>,
        #[arg(long)]
        lane: Option<i32>,
        #[arg(long)]
        amount_ms: Option<i32>,
        #[arg(long)]
        write: bool,
    },
    Bookmarks {
        qua: PathBuf,
        sidecar: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliMacroKind {
    ShiftTime,
    MirrorLanes,
    SelectDensityWindow,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Check { qua } => {
            let map = load_qua(qua)?;
            print_json(&check_map(&map))?;
        }
        Command::Diff { qua } => {
            let map = load_qua(&qua)?;
            let before = std::fs::read_to_string(&qua).unwrap_or_default();
            let after = quaver_qol_core::qua_parser::serialize_qua(&map)?;
            print_json(&diff_text(&before, &after))?;
        }
        Command::Resnap {
            qua,
            snap,
            max_offset_ms,
            include_long_note_ends,
            write,
        } => {
            let mut map = load_qua(&qua)?;
            let report = resnap_map(
                &mut map,
                &ResnapOptions {
                    snap_divisor: snap,
                    max_offset_ms,
                    include_long_note_ends,
                },
            );
            let write_result = write_qua(
                &qua,
                &map,
                &WriteOptions {
                    dry_run: !write,
                    backup_dir: None,
                },
            )?;
            print_json(
                &serde_json::json!({ "report": report, "diff": write_result, "written": write }),
            )?;
        }
        Command::Backup { qua, backup_dir } => {
            print_json(&create_backup(qua, backup_dir.as_deref())?)?;
        }
        Command::Restore { backup, qua } => {
            restore_backup(backup, qua)?;
            print_json(&serde_json::json!({ "restored": true }))?;
        }
        Command::Macro {
            qua,
            kind,
            start_time,
            end_time,
            lane,
            amount_ms,
            write,
        } => {
            let mut map = load_qua(&qua)?;
            let report = apply_macro(
                &mut map,
                &MacroOptions {
                    kind: kind.into(),
                    start_time,
                    end_time,
                    lane,
                    amount_ms,
                },
            );
            let write_result = write_qua(
                &qua,
                &map,
                &WriteOptions {
                    dry_run: !write,
                    backup_dir: None,
                },
            )?;
            print_json(
                &serde_json::json!({ "report": report, "diff": write_result, "written": write }),
            )?;
        }
        Command::Bookmarks { qua, sidecar } => {
            let map = load_qua(&qua)?;
            let extensions = load_bookmark_sidecar(&sidecar)?;
            let merged = merge_bookmark_extensions(&map, &extensions);
            save_bookmark_sidecar(&sidecar, &merged)?;
            print_json(&merged)?;
        }
    }

    Ok(())
}

impl From<CliMacroKind> for MacroKind {
    fn from(value: CliMacroKind) -> Self {
        match value {
            CliMacroKind::ShiftTime => MacroKind::ShiftTime,
            CliMacroKind::MirrorLanes => MacroKind::MirrorLanes,
            CliMacroKind::SelectDensityWindow => MacroKind::SelectDensityWindow,
        }
    }
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
