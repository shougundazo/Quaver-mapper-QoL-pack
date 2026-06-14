import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  AlertTriangle,
  Bookmark,
  FileInput,
  Hammer,
  History,
  Play,
  RotateCcw,
  Save,
  Sparkles,
} from "lucide-react";
import { useMemo, useState } from "react";
import { basename, defaultSidecarPath, issueCounts } from "./lib";
import type { ActionResult, BackupFile, BookmarkPayload, CheckIssue, MapSummary } from "./types";

type Tab = "checker" | "resnap" | "macro" | "bookmarks" | "backups" | "logs";

export default function App() {
  const [summary, setSummary] = useState<MapSummary | null>(null);
  const [issues, setIssues] = useState<CheckIssue[]>([]);
  const [activeTab, setActiveTab] = useState<Tab>("checker");
  const [log, setLog] = useState<string[]>([]);
  const [diff, setDiff] = useState("");
  const [bookmarks, setBookmarks] = useState<BookmarkPayload | null>(null);
  const [backups, setBackups] = useState<BackupFile[]>([]);
  const [snapDivisor, setSnapDivisor] = useState(4);
  const [maxOffset, setMaxOffset] = useState(6);
  const [macroKind, setMacroKind] = useState("shift_time");
  const [amountMs, setAmountMs] = useState(0);
  const [rangeStart, setRangeStart] = useState<number | undefined>();
  const [rangeEnd, setRangeEnd] = useState<number | undefined>();
  const counts = useMemo(() => issueCounts(issues), [issues]);

  async function chooseQua() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Quaver map", extensions: ["qua"] }],
    });
    if (typeof selected !== "string") return;

    const loaded = await invoke<MapSummary>("load_map_summary", { path: selected });
    setSummary(loaded);
    setDiff("");
    setLog((items) => [`Loaded ${basename(selected)}`, ...items]);
    await runCheck(selected);
  }

  async function runCheck(path = summary?.path) {
    if (!path) return;
    const nextIssues = await invoke<CheckIssue[]>("run_checker", { path });
    setIssues(nextIssues);
    setLog((items) => [`Checker found ${nextIssues.length} issue(s)`, ...items]);
  }

  async function runResnap(write: boolean) {
    if (!summary) return;
    const result = await invoke<ActionResult>("run_resnap", {
      payload: {
        path: summary.path,
        snapDivisor,
        maxOffsetMs: maxOffset,
        includeLongNoteEnds: true,
        write,
      },
    });
    setDiff(result.diff.unified);
    setLog((items) => [`Resnap ${write ? "wrote changes" : "dry-run complete"}`, ...items]);
    if (write) await reloadSummary();
  }

  async function runMacro(write: boolean) {
    if (!summary) return;
    const result = await invoke<ActionResult>("run_macro", {
      payload: {
        path: summary.path,
        kind: macroKind,
        startTime: rangeStart,
        endTime: rangeEnd,
        amountMs,
        write,
      },
    });
    setDiff(result.diff.unified);
    setLog((items) => [`Macro ${write ? "wrote changes" : "dry-run complete"}`, ...items]);
    if (write) await reloadSummary();
  }

  async function makeBackup() {
    if (!summary) return;
    await invoke("create_map_backup", { path: summary.path });
    setLog((items) => ["Backup created", ...items]);
    await refreshBackups();
  }

  async function refreshBackups() {
    if (!summary) return;
    const next = await invoke<BackupFile[]>("list_map_backups", { path: summary.path });
    setBackups(next);
    setLog((items) => [`Loaded ${next.length} backup(s)`, ...items]);
  }

  async function loadBookmarks() {
    if (!summary) return;
    const data = await invoke<BookmarkPayload>("load_bookmarks", {
      path: summary.path,
      sidecar: defaultSidecarPath(summary.path),
    });
    setBookmarks(data);
    setLog((items) => [`Loaded ${data.qua.length} .qua bookmark(s)`, ...items]);
  }

  async function reloadSummary() {
    if (!summary) return;
    const loaded = await invoke<MapSummary>("load_map_summary", { path: summary.path });
    setSummary(loaded);
    await runCheck(loaded.path);
  }

  return (
    <main className="shell">
      <aside className="sidebar">
        <button className="primary" onClick={chooseQua}>
          <FileInput size={18} />
          Select .qua
        </button>
        {summary && (
          <div className="mapMeta">
            <strong>{summary.title || basename(summary.path)}</strong>
            <span>{summary.artist || "Unknown artist"}</span>
            <span>{summary.difficultyName || "Unnamed difficulty"}</span>
          </div>
        )}
        <nav>
          <TabButton id="checker" active={activeTab} setActive={setActiveTab} icon={<AlertTriangle size={17} />} />
          <TabButton id="resnap" active={activeTab} setActive={setActiveTab} icon={<Sparkles size={17} />} />
          <TabButton id="macro" active={activeTab} setActive={setActiveTab} icon={<Hammer size={17} />} />
          <TabButton id="bookmarks" active={activeTab} setActive={setActiveTab} icon={<Bookmark size={17} />} />
          <TabButton id="backups" active={activeTab} setActive={setActiveTab} icon={<History size={17} />} />
          <TabButton id="logs" active={activeTab} setActive={setActiveTab} icon={<RotateCcw size={17} />} />
        </nav>
      </aside>

      <section className="workspace">
        <header className="topbar">
          <div>
            <span className="eyebrow">{summary ? basename(summary.path) : "No map selected"}</span>
            <h1>Quaver Mapper QoL Pack</h1>
          </div>
          {summary && (
            <div className="stats">
              <span>{summary.notes} notes</span>
              <span>{summary.timingPoints} timing</span>
              <span>{summary.bookmarks} bookmarks</span>
            </div>
          )}
        </header>

        {!summary ? (
          <div className="empty">
            <FileInput size={36} />
            <button className="primary" onClick={chooseQua}>Select .qua</button>
          </div>
        ) : (
          <>
            {activeTab === "checker" && (
              <section className="panel">
                <div className="panelHeader">
                  <h2>Checker</h2>
                  <button onClick={() => runCheck()}>
                    <Play size={16} /> Run
                  </button>
                </div>
                <div className="counts">
                  <span className="error">{counts.error} errors</span>
                  <span className="warning">{counts.warning} warnings</span>
                  <span>{counts.info} info</span>
                </div>
                <IssueTable issues={issues} />
              </section>
            )}

            {activeTab === "resnap" && (
              <section className="panel">
                <div className="panelHeader">
                  <h2>Resnap</h2>
                  <div className="actions">
                    <button onClick={() => runResnap(false)}>Dry-run</button>
                    <button className="primary" onClick={() => runResnap(true)}>
                      <Save size={16} /> Write
                    </button>
                  </div>
                </div>
                <div className="formGrid">
                  <label>Snap divisor<input type="number" value={snapDivisor} onChange={(e) => setSnapDivisor(Number(e.target.value))} /></label>
                  <label>Max offset ms<input type="number" value={maxOffset} onChange={(e) => setMaxOffset(Number(e.target.value))} /></label>
                </div>
                <DiffView diff={diff} />
              </section>
            )}

            {activeTab === "macro" && (
              <section className="panel">
                <div className="panelHeader">
                  <h2>Selection Macro</h2>
                  <div className="actions">
                    <button onClick={() => runMacro(false)}>Dry-run</button>
                    <button className="primary" onClick={() => runMacro(true)}>
                      <Save size={16} /> Write
                    </button>
                  </div>
                </div>
                <div className="formGrid">
                  <label>Macro<select value={macroKind} onChange={(e) => setMacroKind(e.target.value)}><option value="shift_time">Shift time</option><option value="mirror_lanes">Mirror lanes</option><option value="select_density_window">Select window</option></select></label>
                  <label>Amount ms<input type="number" value={amountMs} onChange={(e) => setAmountMs(Number(e.target.value))} /></label>
                  <label>Start<input type="number" onChange={(e) => setRangeStart(optionalNumber(e.target.value))} /></label>
                  <label>End<input type="number" onChange={(e) => setRangeEnd(optionalNumber(e.target.value))} /></label>
                </div>
                <DiffView diff={diff} />
              </section>
            )}

            {activeTab === "bookmarks" && (
              <section className="panel">
                <div className="panelHeader">
                  <h2>Bookmarks</h2>
                  <button onClick={loadBookmarks}><Bookmark size={16} /> Load</button>
                </div>
                <p className="note">Quaver Bookmarks in the .qua file are the source of truth. Sidecar JSON stores only label, color, memo, and category extensions; missing .qua matches are marked orphan.</p>
                <div className="table">
                  <div className="row bookmarkHead"><span>Time</span><span>Note</span><span>Extension</span><span>Status</span></div>
                  {(bookmarks?.qua ?? []).map((bookmark) => {
                    const extension = bookmarks?.extensions.find((item) => item.startTime === bookmark.startTime);
                    return (
                      <div className="row bookmarkHead" key={`${bookmark.startTime}-${bookmark.note}`}>
                        <span>{bookmark.startTime}</span>
                        <span>{bookmark.note}</span>
                        <span>{extension?.label || extension?.category || ""}</span>
                        <span>{extension?.orphan ? "orphan" : "ok"}</span>
                      </div>
                    );
                  })}
                  {(bookmarks?.extensions ?? []).filter((item) => item.orphan).map((item) => (
                    <div className="row bookmarkHead" key={`orphan-${item.startTime}`}>
                      <span>{item.startTime}</span>
                      <span>{item.label || ""}</span>
                      <span>{item.category || item.memo || ""}</span>
                      <span className="warning">orphan</span>
                    </div>
                  ))}
                </div>
              </section>
            )}

            {activeTab === "backups" && (
              <section className="panel">
                <div className="panelHeader">
                  <h2>Backups</h2>
                  <div className="actions">
                    <button onClick={refreshBackups}><History size={16} /> Refresh</button>
                    <button className="primary" onClick={makeBackup}><Save size={16} /> Create</button>
                  </div>
                </div>
                <p className="note">Every non-dry-run write creates a backup under .quaver-qol-backups next to the map.</p>
                <div className="table">
                  <div className="row backupHead"><span>File</span><span>Size</span><span>Modified</span></div>
                  {backups.map((backup) => (
                    <div className="row backupHead" key={backup.path}>
                      <span>{backup.fileName}</span>
                      <span>{backup.sizeBytes} bytes</span>
                      <span>{backup.modifiedAt || ""}</span>
                    </div>
                  ))}
                </div>
              </section>
            )}

            {activeTab === "logs" && (
              <section className="panel">
                <div className="panelHeader"><h2>Logs</h2></div>
                <ol className="logs">{log.map((item, index) => <li key={`${item}-${index}`}>{item}</li>)}</ol>
              </section>
            )}
          </>
        )}
      </section>
    </main>
  );
}

function TabButton({ id, active, setActive, icon }: { id: Tab; active: Tab; setActive: (tab: Tab) => void; icon: React.ReactNode }) {
  return (
    <button className={active === id ? "active" : ""} onClick={() => setActive(id)} title={id}>
      {icon}
      {id}
    </button>
  );
}

function IssueTable({ issues }: { issues: CheckIssue[] }) {
  return (
    <div className="table">
      <div className="row head"><span>Severity</span><span>Time</span><span>Lane</span><span>Issue</span></div>
      {issues.map((issue, index) => (
        <div className="row" key={`${issue.code}-${index}`}>
          <span className={issue.severity}>{issue.severity}</span>
          <span>{issue.startTime ?? ""}</span>
          <span>{issue.lane ?? ""}</span>
          <span>{issue.message}</span>
        </div>
      ))}
    </div>
  );
}

function DiffView({ diff }: { diff: string }) {
  return <pre className="diff">{diff || "Dry-run output appears here."}</pre>;
}

function optionalNumber(value: string) {
  return value.trim() === "" ? undefined : Number(value);
}
