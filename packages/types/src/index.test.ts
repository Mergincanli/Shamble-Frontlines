import { expect, test } from "vitest";
import * as types from "./index.ts";
import type { AimPoint, Command, Input, Join, JoinAccepted, JoinRejected, RejectReason } from "./index.ts";
import type { LayoutCheck, OwnState, Snapshot, SnapshotAck, ViewTime } from "./index.ts";

test("the package exports the layout, its hash and the check", () => {
  expect(Object.keys(types).sort()).toEqual(["BLOCK_LAYOUT", "LAYOUT_HASH", "checkLayoutHash"]);
  expect(Object.keys(types.BLOCK_LAYOUT)).toEqual(["header", "echo", "link"]);
  expect(types.BLOCK_LAYOUT.header.fields.layout_hash).toEqual({ offset: 4, type: "u32" });
  // Each section starts where the one before it ends.
  const { header, echo, link } = types.BLOCK_LAYOUT;
  expect([header.offset, echo.offset, link.offset]).toEqual([0, header.stride, header.stride + echo.stride]);
  const check: LayoutCheck = types.checkLayoutHash(new ArrayBuffer(0));
  expect(check.ok).toBe(false);
});

// Only the type check (tsc) proves what this test is for: that each message type is exported and has the shape its
// Rust type gives. Vitest strips types, so here it can only fail on the three values checked at the end.
test("values of the eleven message types build (their shapes are proven by tsc, not by this run)", () => {
  const hash = Array.from({ length: 32 }, () => 0) as Join["content_hash"];
  const join: Join = { token: [1, 2, 3], protocol_version: 1, content_hash: hash };
  const accepted: JoinAccepted = {
    slot: 0,
    team: 1,
    squad: 2,
    mode_id: "mode",
    mode_hash: hash,
    content_hash: join.content_hash,
    server_tick: 90,
    tick_rate_hz: 30,
    snapshot_rate_hz: 30,
  };
  const aim: AimPoint = { x: -100, y: 150, z: 2500 };
  const view_time: ViewTime = { tick: 88, fraction: 32768 };
  const presses = Array.from({ length: 16 }, () => 0) as Command["presses"];
  const command: Command = { tick: 91, sequence: 7, move_x: 127, move_y: -127, buttons: 1, presses, aim, view_time };
  const ack: SnapshotAck = { group: 0, tick: 90 };
  const input: Input = { acks: [ack], commands: [command] };
  const own: OwnState = { buttons: command.buttons, presses };
  const snapshot: Snapshot = {
    server_tick: 92,
    baseline_tick: null,
    newest_command_received: command.sequence,
    last_processed_command: null,
    group: ack.group,
    own,
  };
  const reason: RejectReason = "RoomFull";
  const rejected: JoinRejected = { reason };
  expect(rejected.reason).toBe("RoomFull");
  expect(snapshot.own?.presses).toHaveLength(16);
  expect(accepted.content_hash).toHaveLength(32);
  expect(input.commands[0]?.aim).toBe(aim);
  expect(snapshot.baseline_tick).toBeNull();
});
