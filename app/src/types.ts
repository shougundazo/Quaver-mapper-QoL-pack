export type MapSummary = {
  path: string;
  title?: string;
  artist?: string;
  difficultyName?: string;
  mode?: string;
  notes: number;
  timingPoints: number;
  scrollVelocities: number;
  bookmarks: number;
};

export type CheckIssue = {
  severity: "info" | "warning" | "error";
  code: string;
  message: string;
  startTime?: number;
  lane?: number;
};

export type DiffSummary = {
  addedLines: number;
  removedLines: number;
  changed: boolean;
  unified: string;
};

export type ActionResult = {
  report: unknown;
  diff: DiffSummary;
  written: boolean;
};

export type QuaBookmark = {
  startTime: number;
  note: string;
};

export type BookmarkExtension = {
  startTime: number;
  label?: string;
  color?: string;
  memo?: string;
  category?: string;
  orphan: boolean;
};

export type BookmarkPayload = {
  qua: QuaBookmark[];
  extensions: BookmarkExtension[];
};

export type BackupFile = {
  path: string;
  fileName: string;
  sizeBytes: number;
  modifiedAt?: string;
};
