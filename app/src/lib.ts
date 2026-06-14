import type { CheckIssue } from "./types";

export function issueCounts(issues: CheckIssue[]) {
  return issues.reduce(
    (acc, issue) => {
      acc[issue.severity] += 1;
      return acc;
    },
    { error: 0, warning: 0, info: 0 },
  );
}

export function basename(path: string) {
  return path.split(/[\\/]/).pop() || path;
}

export function defaultSidecarPath(quaPath: string) {
  return quaPath.replace(/\.qua$/i, ".quaver-qol.bookmarks.json");
}
