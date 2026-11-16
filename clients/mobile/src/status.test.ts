import { describe, expect, it } from "vitest";
import { statusLabel } from "./status";

describe("statusLabel", () => {
  it("shows status and version", () => {
    expect(statusLabel({ service: "api", status: "ok", version: "0.1.0" })).toBe("ok (v0.1.0)");
  });
});
