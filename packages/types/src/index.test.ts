import { expect, test } from "vitest";
import * as types from "./index.ts";
import type { AimPoint, Command, Input, Join, JoinAccepted, JoinRejected, RejectReason } from "./index.ts";
import type { LayoutCheck, OwnState, Snapshot, SnapshotAck, ViewTime } from "./index.ts";
import type { Bounds, Ground, MapDocument, Movement, Region, Shape, Spawn, Toward } from "./index.ts";

test("the package exports the layout, its hash and the check", () => {
  expect(Object.keys(types).sort()).toEqual(["BLOCK_BYTES", "BLOCK_LAYOUT", "LAYOUT_HASH", "checkLayoutHash"]);
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

// As above: tsc proves the shapes. A content file is JSON, so these are a file's own keys: a shape's `type` and a
// ramp's `rises_toward` stand beside its corners, and the fixed words are the file's own.
test("values of the eight content types build as a content file writes them (their shapes are proven by tsc)", () => {
  const bounds: Bounds = { min: [-32, 0, -24], max: [80, 20, 24] };
  const spawn: Spawn = { id: "street_1", at: [-20, 0, -0.9], facing_deg: 90 };
  const toward: Toward = "-z";
  const box: Shape = { id: "step", min: [30, 0, -10], max: [32, 0.55, -8], type: "box" };
  const ramp: Shape = { id: "roof", min: [-12, 2.6, -5.8], max: [-5.9, 4.7, -3.6], type: "ramp", rises_toward: toward };
  // @ts-expect-error a ramp says which side it rises toward
  const flat: Shape = { id: "roof", min: [0, 0, 0], max: [1, 1, 1], type: "ramp" };
  // @ts-expect-error a shape is a box or a ramp
  const wedge: Shape = { id: "roof", min: [0, 0, 0], max: [1, 1, 1], type: "wedge" };
  // @ts-expect-error a corner has three numbers
  const thin: Shape = { id: "wall", min: [0, 0], max: [1, 1, 1], type: "box" };
  const ground: Ground = "street";
  const region: Region = { id: "corner_street", kind: ground, min: [-32, -2.15], max: [20, 2.15] };
  const map: MapDocument = { id: "test-scene", bounds, spawns: [spawn], shapes: [box, ramp], regions: [region] };
  const movement: Movement = {
    id: "test-soldier",
    walk_speed_m_per_s: 5,
    sprint_multiplier: 1.5,
    speed_up_time_to_90pct_s: 0.2,
    stop_time_to_10pct_s: 0.2,
    air_control_pct: 100,
    body_radius_m: 0.5,
    jump_reach_m: 3,
    gravity_m_per_s2: 20,
    step_up_m: 0.5,
    sprint_time_from_full_s: 6,
    stamina_refill_time_s: 4,
    sprint_returns_at_pct: 25,
    push_strength_per_s: 10,
    push_radius_m: 0.5,
    ads_speed_multiplier: 0.5,
    body_height_m: 1.75,
    foot_radius_m: 0.2,
    pull_share_per_step: 0.25,
    pull_done_under_m: 0.05,
    pull_to_foot_pct: 80,
    step_allowance_m: 0.01,
    snap_down_m: 0.25,
    push_height_gap_m: 0.75,
    push_sideways_share: 0.6,
    push_keep_going_share: 0.4,
    push_most_m_per_step: 0.2,
    push_head_on_gate_m_per_s: 0.3,
    push_keep_going_gate_m_per_s: 0.15,
  };
  expect(map.shapes.map((shape) => shape.type)).toEqual(["box", "ramp"]);
  expect([flat.type, wedge.id, thin.min.length]).toEqual(["ramp", "roof", 2]);
  expect(Object.keys(movement)).toHaveLength(29);
});
