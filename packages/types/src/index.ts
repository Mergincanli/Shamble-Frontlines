// The message types, written by ts-rs from crates/protocol (generated/protocol).
export type { AimPoint } from "./generated/protocol/AimPoint.ts";
export type { Command } from "./generated/protocol/Command.ts";
export type { Input } from "./generated/protocol/Input.ts";
export type { Join } from "./generated/protocol/Join.ts";
export type { JoinAccepted } from "./generated/protocol/JoinAccepted.ts";
export type { JoinRejected } from "./generated/protocol/JoinRejected.ts";
export type { OwnState } from "./generated/protocol/OwnState.ts";
export type { RejectReason } from "./generated/protocol/RejectReason.ts";
export type { Snapshot } from "./generated/protocol/Snapshot.ts";
export type { SnapshotAck } from "./generated/protocol/SnapshotAck.ts";
export type { ViewTime } from "./generated/protocol/ViewTime.ts";

// The content kinds' types, written by ts-rs from crates/content (generated/content): what a content file holds
// beside its `format` and `version`, by the file's own key names.
export type { Bounds } from "./generated/content/Bounds.ts";
export type { Ground } from "./generated/content/Ground.ts";
export type { MapDocument } from "./generated/content/MapDocument.ts";
export type { Movement } from "./generated/content/Movement.ts";
export type { Region } from "./generated/content/Region.ts";
export type { Shape } from "./generated/content/Shape.ts";
export type { Spawn } from "./generated/content/Spawn.ts";
export type { Toward } from "./generated/content/Toward.ts";

// The block's layout and its hash, written by our own generator from crates/client-endpoint (generated/block).
export { BLOCK_BYTES, BLOCK_LAYOUT, LAYOUT_HASH } from "./generated/block/layout.ts";

export { checkLayoutHash } from "./layout-check.ts";
export type { LayoutCheck } from "./layout-check.ts";
