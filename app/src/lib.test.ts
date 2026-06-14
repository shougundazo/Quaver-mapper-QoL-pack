import { describe, expect, it } from "vitest";
import { basename, defaultSidecarPath, issueCounts } from "./lib";

describe("app helpers", () => {
  it("counts issue severities", () => {
    expect(
      issueCounts([
        { severity: "error", code: "a", message: "A" },
        { severity: "warning", code: "b", message: "B" },
        { severity: "warning", code: "c", message: "C" },
      ]),
    ).toEqual({ error: 1, warning: 2, info: 0 });
  });

  it("extracts basename from windows paths", () => {
    expect(basename("C:\\maps\\song.qua")).toBe("song.qua");
  });

  it("derives sidecar path", () => {
    expect(defaultSidecarPath("/maps/song.qua")).toBe("/maps/song.quaver-qol.bookmarks.json");
  });
});
