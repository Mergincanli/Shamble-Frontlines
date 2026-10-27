# R2 unit rooms: rooms and private matches

Brief question (quoted): "rooms and private matches" (V2_PHASE0_BRIEF §4, row R2). Unit scope as assigned: the room and
match lifecycle (create, fill, start, end, tear down); one process per room versus many rooms per process at 48+
players; joining with a short-lived signed token from a lobby, and reconnecting; private matches with room codes,
including VISION_V2 §3's organized play; bots filling empty slots on the server; modes, team count, team size and
factions as data (VISION_V2 §6.1); guest play first (VISION_V2 §5); what a spike must prove.

Author: netcode-architect, 2026-10-27. Status: unit draft (not verified). Every source read 2026-10-27. "Confirmed in
the source" means the claim rests on sentences quoted verbatim below; "inferred" means a conclusion drawn from them,
or a claim seen only as a WebFetch paraphrase.

## 1. Findings

**Room and match lifecycle (hosting services)**

- **R2-ROOM-F1.** In Amazon GameLift Servers, a server process reports ready, the service picks a process that is not
  hosting a game session, sets the session to ACTIVATING, and the process calls `ActivateGameSession()` only when it can
  accept players; one process hosts one game session at a time. Verbatim: "`ProcessReady()` to communicate readiness to
  host a game session."; "searches those fleets for an active server process that's not hosting a game session";
  "Changes the server process's number of game sessions to `1`."; "When the server process is ready to accept player
  connections, it calls the server SDK operation `ActivateGameSession()` and waits for player connections." ·
  https://docs.aws.amazon.com/gameliftservers/latest/developerguide/gamelift-sdk-interactions.html, "Initializing a
  game server process" and "Creating a game session" · official · read 2026-10-27 · confirmed in the source (the
  one-session-per-process reading is inferred from the two quoted sentences).
- **R2-ROOM-F2.** GameLift reserves a player slot before the player connects and the game server validates the player
  on connect; an unvalidated reservation times out after 60 s and reopens the slot; a dropped player's session becomes
  COMPLETED. Verbatim: "Creates a new `PlayerSession` object and sets the status to `RESERVED`."; "the game server
  process calls the service API operation `AcceptPlayerSession()` to validate the player session ID. The server process
  either accepts or rejects the connection."; "TIMEDOUT -- A player session request was received, but the player did
  not connect and/or was not validated within the timeout limit (60 seconds)."; "COMPLETED -- The player connection has
  been dropped." · S1 ("Adding a player to a game", "Removing a player") and
  https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_PlayerSession.html, "Status" (API version
  gamelift-2015-10-01) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F3.** GameLift ends a session by the process ending: the server ends the session and calls
  `ProcessEnding()`; the service then marks session and process TERMINATED and may start a new process. Health checks
  must be answered within one minute, and three consecutive unhealthy reports shut a process down. A session that does
  not become active within the activation timeout (1 to 600 s) is terminated. Verbatim: "The game server process ends
  the game session and initiates process shut down by calling the server SDK operation `ProcessEnding()`."; "The server
  process must respond healthy or not healthy within one minute."; "Amazon GameLift Servers automatically shuts down
  server processes that are reported unhealthy for three consecutive reports."; "If the game session does not become
  active before the timeout, it is ended and the game session status is changed to `TERMINATED`." · S1 "Shutting down
  the game session"; https://docs.aws.amazon.com/gameliftservers/latest/developerguide/fleets-multiprocess.html
  "Optimize a fleet for multiple processes"; https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_RuntimeConfiguration.html
  "GameSessionActivationTimeoutSeconds" · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F4.** GameLift packs density by running several processes per instance, not several sessions per process;
  the right count is found by testing. Verbatim: "You can set up a managed EC2 fleet's runtime configuration to run
  multiple game server processes per instance. This uses your hosting resources more efficiently."; "An Amazon GameLift
  Servers instance is limited to 50 processes running concurrently."; "Test different configurations for your game to
  find the right number of processes for your preferred instance type." · S4 (intro, "Choose the number of processes
  per instance"); S3 (description) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F5.** PlayFab Multiplayer Servers uses the same model: a server is allocated once (StandingBy to Active),
  is never returned to the pool, is protected from scale-down while Active, and the process should exit when the
  session ends. Verbatim: "A Game server can never transition back to StandingBy state when it is in the Active state.
  Once the game state is set to Active, the only way to get a new game server in StandingBy state is if your game server
  process exits."; "The fact that the game server is active means that it will not be taken down during potential Build
  scale downs"; "When the game session ends, you should gracefully terminate the game server process so you can get a
  new game server." · https://learn.microsoft.com/en-us/gaming/playfab/multiplayer/servers/multiplayer-game-server-lifecycle,
  "States of a game server", "Terminating a game server process" (ms.date 2022-01-26, updated 2025-05-01) · official ·
  read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F6.** Agones (the googleforgames/agones project's docs) documents many sessions per game server as a
  cheaper but harder pattern; its Beta "rooms" Counter lets allocation prefer an already-allocated server with spare
  room capacity. Verbatim (as quoted by WebFetch): "it can be a more economical use of resources to run multiple
  concurrent game sessions"; "this requires more management on behalf of the integrated game server process and
  external systems"; "since it works around the common Kubernetes and/or Agones container lifecycle." ·
  https://raw.githubusercontent.com/googleforgames/agones/main/site/content/en/docs/Integration%20Patterns/high-density-gameservers.md
  (main branch; front-matter date 2021-08-31; Counters marked Beta, gate CountsAndLists) · official (project docs) ·
  read 2026-10-27 · confirmed in the source for the quoted sentences; the Counter mechanics are inferred (paraphrase).

**Many rooms in one Rust process**

- **R2-ROOM-F7.** Tokio tasks are cooperative, and blocking work stalls every task on that worker thread;
  `spawn_blocking` is for work that ends, and work that runs indefinitely belongs on its own thread. Verbatim: "a task
  is allowed to run until it yields"; a blocking operation "would block the entire thread, preventing other tasks from
  running"; `spawn_blocking` is "intended for non-async operations that eventually finish on their own"; for "workloads
  that run indefinitely or for extended periods" use `thread::spawn`; "tasks spawned using spawn_blocking cannot be
  aborted because they are not async." · https://docs.rs/tokio/latest/tokio/task/index.html and
  https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html (tokio 1.54.1) · official · read 2026-10-27 ·
  confirmed in the source (quoted fragments); that the page names `thread::spawn` for long-running work is from
  WebFetch's paraphrase around the quote, so that link is inferred.
- **R2-ROOM-F8.** In Rust an uncaught panic ends only its own thread and is reported to `join`, unless the binary is
  built with `panic = "abort"`, which terminates the whole process; the default thread stack is 2 MiB. Verbatim: "If
  the panic is not caught the thread will exit, but the panic may optionally be detected from a different thread with
  `join`."; panics "can nonetheless be caught (unless compiling with `panic=abort`) with `catch_unwind`"; Cargo
  `"abort"`: "Terminate the process upon panic."; "The default stack size is platform-dependent and subject to change.
  Currently, it is 2 MiB on all Tier-1 platforms." · https://doc.rust-lang.org/std/thread/index.html "Panics", "Stack
  size" (page shows Rust 1.99.1, 2026-10-13); https://doc.rust-lang.org/cargo/reference/profiles.html "panic" ·
  official · read 2026-10-27 · confirmed in the source.

**Joining with a token, and reconnecting**

- **R2-ROOM-F9.** Glenn Fiedler's browser-game design: a web backend authenticates the player and returns a short-lived
  connect token over HTTPS; it is signed and encrypted with a key shared by the backend and the game servers, lists the
  server addresses it is valid for, and a server rejects a token it has already seen and a second connection with the
  same client id. Verbatim: "the web backend performs authentication and when a client wants to play, it makes a REST
  call to obtain a connect token"; "Connect tokens are short lived"; "checks if the connect token has already been used
  by searching a short history of connect token HMACs"; "This prevents one connect token from being used to connect
  multiple clients."; "only one client by unique client id may be connected at any time." ·
  https://gafferongames.com/post/why_cant_i_send_udp_packets_from_a_browser/ (2017-02-26) · shipped game (engineer's
  articles, named in the brief) · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F10.** The netcode 1.02 standard makes a connect token single-use and tells a rejected client to fetch a new
  one; a server ignores a token that is expired, not addressed to it, or for a client id already connected. Verbatim:
  "A connect token is therefore usable for exactly one connection."; "The correct client response is to request a new
  connect token from the web backend."; "If the connect token expire timestamp is <= the current timestamp, ignore the
  packet."; "If a client with the client id contained in the private connect token data is already connected, ignore
  the packet." · https://raw.githubusercontent.com/mas-bandwidth/netcode/main/STANDARD.md (netcode 1.02, main branch) ·
  shipped game (engineer's protocol spec) · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F11.** RFC 7519 defines the claims such a token needs: an audience the receiver must match, an expiry with
  small clock-skew leeway, and a unique id. Verbatim: "Each principal intended to process the JWT MUST identify itself
  with a value in the audience claim." ... "then the JWT MUST be rejected."; "identifies the expiration time on or after
  which the JWT MUST NOT be accepted for processing."; "Implementers MAY provide for some small leeway, usually no more
  than a few minutes, to account for clock skew."; "The identifier value MUST be assigned in a manner that ensures that
  there is a negligible probability that the same value will be accidentally assigned to a different data object" ·
  https://www.rfc-editor.org/rfc/rfc7519.html §4.1.3, §4.1.4, §4.1.7 (May 2015) · official · read 2026-10-27 · confirmed
  in the source.
- **R2-ROOM-F12.** Neither browser transport lets the page set an HTTP auth header: the WebSocket constructor takes only
  a URL and sub-protocols, and the WebTransport constructor's options are `allowPooling`, `congestionControl`,
  `protocols`, `requireUnreliable` and `serverCertificateHashes`; a WebTransport URL must be https with an explicit port.
  Verbatim: "`new WebSocket(url)` `new WebSocket(url, protocols)`"; "The scheme must be HTTPS, and the port number needs
  to be explicitly specified." · https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket (modified
  2026-06-22) and https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport (modified 2026-10-12) ·
  official · read 2026-10-27 · confirmed in the source for the signatures and option list; "no auth header" is inferred
  (neither page says it in words).
- **R2-ROOM-F13.** A WebTransport session is an extended CONNECT whose `:authority` and `:path` name the server
  resource, so one endpoint can tell sessions apart by path; servers should rate-limit new sessions. Verbatim: "In order
  to create a new WebTransport session, a WebTransport client sends an HTTP extended CONNECT request."; "Both the
  :authority and the :path value MUST be set; these fields identify the desired WebTransport server resource.";
  "Servers SHOULD limit the rate of incoming WebTransport sessions on HTTP/3 connections." ·
  https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/ (draft-ietf-webtrans-http3-16, 2026-07-06, work in
  progress) §3.2, §5.2 · official (IETF draft) · read 2026-10-27 · confirmed in the source; routing rooms by path is
  inferred.
- **R2-ROOM-F14.** A guessable secret needs at least 64 bits of entropy, and identifiers in URLs leak through logs,
  history and the Referer header. Verbatim: "Session identifiers must have at least 64 bits of entropy to prevent
  brute-force session guessing attacks."; "Each "bit" of entropy doubles the number of possible outcomes."; IDs in the
  URL "might disclose the session ID (in web links and logs, web browser history and bookmarks, the Referer header or
  search engines)."; "Sensitive data like the session ID should not be included in the logs" ·
  https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html "Session ID Entropy" and its passages on IDs in URLs and on logging (no
  page date) · official (OWASP) · read 2026-10-27 · confirmed in the source.

**Bots, guests and private rooms**

- **R2-ROOM-F15.** Team Fortress 2's server fills a quota with bots: in 'fill' mode bots make up the gap to N players,
  bots can wait for the first human, and bots leave to make room for humans. Verbatim (ConVar help strings): "If
  'fill', the server will adjust bots to keep N players in the game, where N is bot_quota."; "If 'match', the server
  will maintain a 1:N ratio of humans to bots, where N is bot_quota."; "If nonzero, bots wait until a player joins
  before entering the game."; "If nonzero, bots will automatically leave to make room for human players."; code:
  `desiredBotCount = MAX( 0, desiredBotCount - nNonTFBotsOnGameTeams );` ·
  https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/tf/bot/tf_bot_manager.cpp ·
  shipped game (source code) · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F16.** PlayFab recommends an anonymous login for new players (no input), then offering a recoverable
  login; an anonymous account is lost with the device, and a simple custom id lets attackers take accounts. Verbatim:
  "They require zero input from the player, so there's no friction to the first time user experience"; "Your game
  should use an anonymous login for creating a new account and linking new devices to an existing account."; "once the
  anonymous login is complete, you should provide the option to add recoverable login credentials"; "If the player
  loses or breaks their device, the account is lost"; "you must generate your custom IDs with care. If they're too
  simple, you risk hackers stealing the accounts of others." ·
  https://learn.microsoft.com/en-us/xbox/playfab/identity/player-identity/login/login-basics-best-practices (ms.date
  2025-02-20, updated 2025-10-31) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F17.** PlayFab Lobby fixes a lobby's capacity, access policy, owner and owner-migration policy at creation;
  the owner alone updates lobby data; under the Automatic policy ownership passes to another connected member; a
  client-owned lobby is deleted when its last member leaves. Verbatim: "Several important settings for a lobby are
  configured during creation: maxMemberCount, accessPolicy, owner, and ownerMigrationPolicy."; "accessPolicy defines
  who may discover the lobby's connection string"; "owner defines the lobby's owner which has special permissions to
  update data on the lobby."; "The Lobby service automatically assigns another connected member as owner when the
  current owner leaves or disconnects."; "For client-owned lobby, the lobby is always deleted when the last member
  leaves." · https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/create-a-lobby (updated
  2026-03-24) and https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/ownership-changes
  (updated 2025-09-10) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F18.** In the process-per-session model each session is reached at its own address and port. Verbatim:
  "When joining a game session, the game client connects directly to a game session using the game session's unique IP
  address and port number." · S1 (introduction) · official · read 2026-10-27 · confirmed in the source.

## 2. Legacy facts used (docs/LEGACY_INVENTORY.md rows; docs/LEGACY_TUNING.md tables)

| Row / table | Fact | What it tells v2 rooms |
|---|---|---|
| CD10, `TEAMS` table | Three teams as data (allies, soviets, empire); an implicit "order matters" rule makes soviets the first enemy. | Teams already data; a mode must name its teams and factions explicitly, never by table order. |
| CD11, `TUNE` table | `teams.size` 4, `ff` false; bot sight budget 480 checks/s, sized for 12 soldiers (STATUS at f2ccc13 :68). | Team size and friendly fire are mode data; the shared bot budget must be re-sized per room for 48+. |
| GC2 | 12 fixed slots (0 = you, 1-3 allies, 4-7 soviets, 8-11 empire); slot id = SIM id; unused teams parked when they have no HQ; perfbots at 48 soldiers: combat 0.80-1.00 ms per update step (JS, headless). STATUS :68: "humans take over bot slots". | Keep a slot table with slot id = sim id, sized from the mode; park unused teams by the mode, not "has an HQ"; the 48-soldier JS number is the only CPU figure we have (order of magnitude only). |
| GN2, GN3, GC9 | Bot records mix sim and view state; unit picked with `Math.random`; per-slot deterministic spawn chain; respawn 2.0 s / 4.0 s (`T` table). | Bots become sim-owned slot occupants on the server with seeded randomness; spawn rules come from LOOP_V2 §2. |
| GW1 | `PLAYER_FACTION` / `PLAYER_TEAM` globals fix one local player to the Allies. | Team and squad become per-connection state assigned by the room. |
| GW2 / GE1, `RULES_DEFAULT` | Rules are per-map data merged over defaults; the merge accepts any key, including `__proto__` (SEC-MDL-2). | Mode and rules data must be schema-validated by the server at load. |
| GW11 | The boot map comes from the URL (`?map=tri`). | The room (server) chooses the map from the mode, never the client. |
| GE12 | `resetRound` mutates many globals in place; `gameOver` freezes the world. | A room should start each match from a fresh sim instance, not an in-place reset. |
| GP13, PG8 | Pause, shop and tutorial stop `update()`; "Restart round" is a player button. | A room never pauses for one player; restart and end belong to the room (or the private-room host). |
| CS1 | A 1/60 s fixed step and an input-command record (seq, tick, move, aim, buttons) exist but nothing reads them. | Bots and humans can share one input-command path into the sim. |
| GC1 | One team choke point; the server must repeat the team allow-list. | The room validates every team id in joins and host commands. |
| CD2, GE15 | Career lives in localStorage (KVStore), loaded with no validation. | Client storage keeps only the guest credential and settings; progression is server-side (VISION §5, §6.10 LATER). |

## 3. Options

### 3.1 Where a room runs (process model)

| Option | Isolation | CPU and memory at 48+ | Ports, routing, deploys | Complexity and risk | Fit |
|---|---|---|---|---|---|
| A. One OS process per room (GameLift and PlayFab default: F1, F4, F5) | Strongest: a crash, leak or runaway loop ends one room. | One sim thread per room plus a small I/O runtime; per-process baseline memory unknown (unverified; spike measures). Density comes from more processes per host (F4). | One port per room (F18); the lobby hands out host:port; WebTransport URLs carry an explicit port anyway (F12). Teardown = process exit, so nothing accumulates across matches. Drain = stop starting new rooms. | Lowest: lifecycle equals process lifetime; needs a small supervisor (R6). | §6.1 rooms; §4 fairness (no noisy neighbour inside a process). |
| B. Many rooms per process, one dedicated OS thread per room sim, shared async network I/O | Partial: a panic stays in its thread only with `panic = "unwind"` (F8); an abort, memory corruption in unsafe code, out-of-memory or a stuck lock still takes all rooms. | Shared baseline; each room thread has a 2 MiB default stack (F8); one runtime for all sockets. | One port and one certificate for all rooms; rooms routed by WebTransport `:path` (F13). A deploy must drain every room in the process. | Higher: room registry, per-room accounting, crash containment, "more management" (F6). | §6.1; cheaper for many tiny rooms. |
| C. Many rooms as tasks on one shared async runtime | Weakest. | A room's tick loop is long-running CPU work; on a tokio worker it stalls other tasks (F7). | As B. | Tick jitter across rooms; ruled out by F7. | Poor. |

### 3.2 How a player is admitted

| Option | How | Trade-offs |
|---|---|---|
| A. Signed join token checked offline by the room | Lobby signs {player id, guest flag, room id (audience), team or slot if pre-assigned, expiry, unique id}; room checks signature, audience, expiry, unused id (F9, F10, F11). | No lobby call on the join path; keys must be shared or published; replay blocked by the used-id history (F9, F10). |
| B. netcode.io-style encrypted token (F9, F10) | As A, but the private part is AEAD-encrypted with a key shared by lobby and servers. | Hides the contents and can carry per-connection keys; every server holds a key that can mint tokens. |
| C. Opaque ticket validated by a call back to the lobby (GameLift `AcceptPlayerSession`, F2) | Room asks the lobby "is ticket X valid for room R?" | Simplest crypto; adds a round trip and makes the lobby a live dependency of every join. |

Token carriage: neither browser API sets an auth header (F12). The token can go in the URL (leaks into logs and
history, F14), in the sub-protocol list, or in the first message on a reliable stream; the last keeps it out of URLs
and access logs (inferred from F12 and F14).

### 3.3 Reconnecting

- **A. Hold the slot for a grace window; rejoin with a fresh token.** The room keeps the player's slot, team and squad;
  the client fetches a new token for the same room and player id (F10's "request a new connect token"); the room maps
  the player id back to the slot; a newer connection for the same player id replaces the older one (F9's one client per
  client id, used here as replace rather than ignore). One verification path for joins and rejoins.
- **B. Room-issued resume secret.** The room gives each client a resume secret at join; the client presents it to the
  room directly. Saves a lobby round trip; adds a second credential type the room must issue and protect.
- **C. No hold.** A dropped player rejoins as a new player, if a slot is free. Simplest; loses the slot to a bot or to
  another player, and splits the player's match record.
What the player's soldier does during the grace window (stands, is bot-driven, is removed) is a game rule: Mustafa's.

### 3.4 Private matches and room codes

| Option | Trade-offs |
|---|---|
| A. Short room code that only locates a room; joining still needs a lobby token; code lookups rate-limited | Easy to read out over voice; a 6-symbol code from 32 symbols is 32^6 = 1,073,741,824 codes (about 30 bits), far below the 64 bits a secret needs (F14), so it must not be the secret. With 1,000 live private rooms a random guess hits some room with probability about 9.3e-7; at a proposed 10 lookups per minute per client that is about 0.013 hits per client per day (arithmetic, inferred). A host lock and kick bound the harm. |
| B. Long invite link that is itself the credential (at least 64 bits, F14) | Unguessable; but a link is a URL and leaks through logs, history and Referer (F14); poor for voice. |
| C. Both: code to find, link to share | Most convenient; two paths to build and test. |
Room settings: PlayFab fixes capacity, access and owner policy at creation and lets only the owner change room data
(F17). For organized play (VISION §3 [DECIDED]) the creator's settings must at least include the faction of each team;
which other settings a host may change, and when they freeze, is design.

### 3.5 Bots filling slots

- **A. Per-team fill quota on the server:** bots per team = team size − humans on that team, recomputed on every join
  and leave; a human takes a bot's slot (TF2 'fill' and auto-vacate, F15; legacy STATUS :68).
- **B. Ratio ('match', F15):** bots scale with humans. Does not keep teams full, so it does not meet VISION §3's "bots
  fill empty slots".
- **C. Fixed or no bots:** only for private rooms whose host turns bots off, if Mustafa allows it.
When a human replaces a bot (at once, or when the bot next dies) is a game rule.

### 3.6 Modes as data

- **A. Mode files validated at load; a room is a mode id plus the overrides the mode allows.** Team count (2 or 3),
  team size, factions per team, squad size, rules table, map list, bot policy, match length and the allowed private
  overrides are data; the slot table is built from the mode (GC2). Format and hot reload belong to R1's data unit.
- **B. Modes in Rust code.** Simple at first; every new mode or team size is a code change, against VISION §3 [DEFAULT]
  and §6.1.
- **C. Per-map rules only (legacy GW2).** Mixes map and mode; cannot express "same map, 2 or 3 teams".

### 3.7 Guest identity

- **A. Anonymous guest account from the lobby:** zero input; the lobby issues a random guest id and a secret of at
  least 64 bits (F14, F16) that the client keeps in local storage; a login can later be linked to it (F16).
- **B. No identity (a random name per session):** no reconnect across a page reload, no later linking.
- **C. Login required:** against VISION §5 [DECIDED] "guest play first".

## 4. Recommendations

- **R2-ROOM-REC1. An explicit room lifecycle, one fresh simulation per match.** States: Created (mode and map loaded,
  slot table built) → Open (accepting joins; bots fill) → Countdown → Running → Ended (final scores, results and
  telemetry written) → Closed (connections closed, room gone). Each state has a timeout: activation (F3's 1 to 600 s
  pattern), an empty-room timeout, the mode's match length (LOOP_V2 §8: about 20 min). The room answers a supervisor
  health check (F3). A match never pauses or restarts for one player (GP13, PG8), and the next match starts from a new
  sim instance (GE12).
  Reason: the shipped hosting services all model a session as a bounded life with activation, health and an end (F1,
  F3, F5); legacy shows in-place resets and world freezes do not survive multiplayer.
  Rests on: F1, F3, F5. Serves VISION §6.1 (rooms), §6.6 (results and telemetry recorded at Ended).
  Spike must prove (proposed): (1) 100 create → fill with bots → run 60 s → end → close cycles in one host with zero
  leaked rooms, threads, sockets or ports, and memory back within 1 MB of baseline after the cycles; (2) a room stuck in
  Created past its activation timeout is closed and reported; (3) results and telemetry for a match are written before
  Closed in 100 % of normal ends.
- **R2-ROOM-REC2. One room per OS process to start; inside it, the sim on its own thread and network I/O on an async
  runtime.** The supervisor (R6) starts a process per room; the lobby hands out host and port. Option B (many rooms per
  process) is the fallback only if the spike shows per-process memory or start-up cost is too high for the host budget,
  and then needs `panic = "unwind"` with per-room containment.
  Reason: it is the default model of GameLift and PlayFab (F1, F4, F5), it gives the strongest isolation for a fair
  tick (VISION §4), teardown is process exit, and density comes from processes per host (F4); many rooms per process
  is "more economical" but needs "more management" (F6), and rooms must not share an async worker (F7).
  Rests on: F1, F4, F5, F6, F7, F8, F18. Serves VISION §6.1, §4 (fair up to about 100 ms of ping).
  Spike must prove (proposed; reference host proposed: one 4-vCPU EU cloud VM, R6 to name it): (1) one 48-player room
  (bots) holds its tick: p99 tick time at most 50 % of the tick period chosen by the tick-rate unit, over 20 minutes;
  (2) 8 such rooms as 8 processes on the reference host: each room's p99 within 10 % of the single-room figure;
  (3) per-room process memory measured (RSS at start and after 20 minutes) and reported, with a proposed ceiling of
  64 MB for a 48-player room; (4) killing one room's process (SIGKILL) leaves the other rooms' tick p99 and connections
  unchanged; (5) room start-up (process spawn to Open) under 2 s.
- **R2-ROOM-REC3. Join with a short-lived signed token from the lobby, checked by the room offline (option 3.2 A).**
  Claims: player id, guest flag, display name, room id as audience, team/slot if pre-assigned, expiry (proposed 30 s
  after issue, leeway at most 5 s), unique id. The room rejects a token whose audience is not itself, that has expired,
  whose unique id it has already seen within the expiry window, or with a bad signature. The token travels in the first
  message of a reliable stream, never in the URL. The room, not the client, assigns team and squad within the mode's
  rules (GW1, GC1).
  Reason: an offline check keeps the lobby off the hot join path; single use and the audience stop replay and
  cross-room use (F9, F10, F11); browsers cannot send an auth header (F12) and URLs leak (F14).
  Rests on: F9, F10, F11, F12, F14. Serves VISION §6.1. Signature scheme and key handling: open (OQ2).
  Spike must prove (proposed): (1) 1,000 generated bad tokens (expired, wrong room, replayed, altered byte, wrong key,
  missing claim) are all rejected, and 1,000 good ones all accepted; (2) a join completes (transport ready to first
  snapshot) within 1 RTT + 100 ms at 0-200 ms simulated latency with 5 % loss; (3) no token appears in server access
  logs.
- **R2-ROOM-REC4. Reconnect by holding the slot for a grace window and rejoining with a fresh token (option 3.3 A).**
  A dropped player keeps slot, team and squad for a grace window (proposed 60 s, matching F2's reservation timeout as a
  starting value); the client fetches a new token for the same room; the newest connection for a player id wins and the
  older one is closed; after the window the slot follows the bot-fill rule.
  Reason: the netcode standard makes tokens single-use and sends a rejected client back to the backend (F10); a
  reservation with a timeout is how GameLift holds slots (F2); one connection per player id prevents duplicate
  soldiers (F9). Slot id = sim id (GC2) makes the rebind cheap.
  Rests on: F2, F9, F10. Serves VISION §6.1. What the soldier does meanwhile: OQ3.
  Spike must prove (proposed): (1) in the network simulator, 200 forced drops of 1-55 s at 0-200 ms latency, 0-5 %
  loss: 100 % rejoin the same slot and team, with no duplicate soldier; (2) a second tab with the same player id takes
  over within 1 s and the first is closed; (3) a drop longer than the window hands the slot to the fill rule.
- **R2-ROOM-REC5. Private rooms: a short code that only locates the room, a host with server-checked rights, settings
  fixed at start (option 3.4 A).** Codes come from a cryptographically secure random source, are unique among live
  rooms, die with the room, and lookups are rate-limited per client and per address (F13's rate-limit advice applied to
  lookups, inferred); joining still needs a token (REC3). The creator is the host; the room checks every host command
  (start, kick, lock, team and faction settings) against the host's player id. For organized play the settings carry a
  faction per team (VISION §3 [DECIDED]); which settings exist beyond that, and host migration, are design (OQ5, OQ6).
  Reason: a human-readable code is far below a secret's entropy (F14), so it must only locate; PlayFab fixes room
  policy at creation and gives the owner sole write rights (F17).
  Rests on: F14, F17 (F13 for rate limiting). Serves VISION §6.1 (private matches), §3 (organized play).
  Spike must prove (proposed): (1) 10,000 codes generated with no collision among live rooms; (2) lookups above the
  limit (proposed 10 per minute per client) are refused; (3) every host command sent by a non-host is rejected; (4) a
  private room created with factions {Soviets, Allies} on a two-team mode starts with exactly those factions.
- **R2-ROOM-REC6. Bots fill per team on the server and drive the same input path as humans (option 3.5 A).** The slot
  table records for each slot whether it is human, bot or empty; bots per team = team size − humans on that team; a
  joining human takes a bot's slot; bots produce the same input commands a client sends, so the sim cannot tell them
  apart (inferred from the brief's one-simulation principle and legacy CS1, the unused `makeInputCommand` record). The bots' shared sight
  budget scales with the room size (CD11).
  Reason: a shipped game keeps servers full this way (F15); legacy already plans humans taking bot slots (GC2, STATUS
  :68).
  Rests on: F15. Serves VISION §3 ("Bots fill empty slots"), §6.1.
  Spike must prove (proposed): (1) 3 teams × 16 slots, 30 humans (simulated clients) joining and leaving in random
  order for 10 minutes: after every change, within 1 s, each team has exactly its size in humans plus bots and no team
  exceeds it; (2) a human never waits for a slot while a bot holds one on the team they are assigned to; (3) the same
  seed and the same join/leave script give the same per-tick state hash on two runs.
- **R2-ROOM-REC7. Modes as validated data; a room is a mode id plus allowed overrides (option 3.6 A).** The mode
  defines team count, team size, faction per team (or "chosen by host"), squad size, rules table, maps, bot policy and
  match length; the server rejects at load any file with unknown keys, out-of-range values (team count other than 2 or
  3, size 0) or prototype keys (SEC-MDL-2); the slot table and team index are built from the mode (GC2, CD10).
  Reason: VISION §3 [DEFAULT] and §6.1 want all of it as data; legacy shows teams as data work (CD10, GC2) and that
  unchecked merges are a hole (GW2).
  Rests on: legacy CD10, GC2, GW2 (no external finding needed); format per R1. Serves VISION §6.1.
  Spike must prove (proposed): (1) mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24 load and each runs 60 s
  with bots, with no code change between them; (2) 20 malformed mode files are all rejected at load with a named error.
- **R2-ROOM-REC8. Guest first: an anonymous guest account from the lobby (option 3.7 A).** On first visit the lobby
  issues a guest id and a random secret of at least 64 bits; the client stores them (the KVStore successor) and uses
  them to get join tokens; a later login links to the same id. The room only ever sees the join token.
  Reason: zero-input anonymous accounts are the recommended start, with a recoverable login added later (F16); the
  secret must be unguessable (F14, F16).
  Rests on: F14, F16. Serves VISION §6.1 (a player identity for rooms and reconnects) and prepares §6.10 [LATER]
  without building it; VISION §5 [DECIDED].
  Spike must prove (proposed): (1) a fresh browser profile reaches a room with no input beyond "Play"; (2) a page reload
  keeps the same guest id and rejoins the held slot (with REC4); (3) the secret is 128 random bits from the OS source
  (proposed; at least 64 required).

Room control messages (proposed, reliable stream; per-player cost is a few hundred bytes per event, negligible
against snapshots, inferred): `Join{token}` → `JoinAccepted{slot, team, squad, modeId, modeHash, serverTick}` or
`JoinRejected{reason}`; `RoomState{phase, countdown, roster}` on change; `HostCommand{start | kick | lock | settings}`
(private rooms); `RoomEnded{results}`; `Leave`. The lobby speaks HTTPS: guest create, room create (private), code
lookup, token request.

## 5. Open questions

- **R2-ROOM-OQ1. Process per room or many per process, finally?** Decided by REC2's spike numbers (per-process memory,
  start-up, tick p99 with neighbours) against R6's host sizes and costs.
- **R2-ROOM-OQ2. Token signature scheme and keys.** Asymmetric signature (rooms hold only a public key) or a shared key
  with AEAD as in netcode.io (F9, F10)? Key rotation and storage? Not researched here (R6's security baseline is
  deferred by the Advisor; security-auditor reviews R2's protocol). Decided by security-auditor's review.
- **R2-ROOM-OQ3. The soldier of a dropped player during the grace window.** Stands still, bot-driven, or removed? A game
  rule: Mustafa (game-designer can model it).
- **R2-ROOM-OQ4. Joining a match in progress.** May humans join a Running public or private match? Does the bot leave
  at once or at its next death? Team auto-balance? Game rules: Mustafa.
- **R2-ROOM-OQ5. What a private-room host may set.** Faction per team is decided (VISION §3). Team size, bots on or off,
  mode rules, map, how clan members are put on one team (VISION §7 lists clan and guild features as open): Mustafa.
- **R2-ROOM-OQ6. Host leaves.** Pass host automatically to another connected human (PlayFab "Automatic", F17), or leave
  the room hostless? Design, with the engine cost small either way.
- **R2-ROOM-OQ7. Do private matches count?** Service points (LOOP_V2 §8) and balance telemetry (§6.6): a tag on the
  match record is cheap; whether they count is Mustafa's call.
- **R2-ROOM-OQ8. Can WebTransport survive an address change without a reconnect?** QUIC has connection migration;
  browser support for it in WebTransport was not researched here (transport unit). It changes how often REC4 runs, not
  its design.
- **R2-ROOM-OQ9. Where the lobby runs and stores guest ids and the room directory.** Same binary as the supervisor or a
  separate service; which store: R6.
- **R2-ROOM-OQ10. Code length and alphabet.** 6 symbols from 32 is a proposal; whether a password is also offered for
  private rooms is design plus security-auditor.

## 6. Sources read (all read 2026-10-27)

1. https://docs.aws.amazon.com/gameliftservers/latest/developerguide/gamelift-sdk-interactions.html: "Game
   client/server interactions with Amazon GameLift Servers" (no page date).
2. https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_PlayerSession.html: "PlayerSession" (API
   gamelift-2015-10-01).
3. https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_RuntimeConfiguration.html: "RuntimeConfiguration"
   (API gamelift-2015-10-01).
4. https://docs.aws.amazon.com/gameliftservers/latest/developerguide/fleets-multiprocess.html: "Optimize game server
   runtime configuration on managed Amazon GameLift Servers" (no page date).
5. https://learn.microsoft.com/en-us/gaming/playfab/multiplayer/servers/multiplayer-game-server-lifecycle: "Lifecycle
   of a multiplayer server - PlayFab" (2022-01-26, updated 2025-05-01).
6. https://raw.githubusercontent.com/googleforgames/agones/main/site/content/en/docs/Integration%20Patterns/high-density-gameservers.md:
   Agones "High Density GameServers" (main; 2021-08-31).
7. https://docs.rs/tokio/latest/tokio/task/index.html: "Module tokio::task" (tokio 1.54.1).
8. https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html: "spawn_blocking" (tokio 1.54.1).
9. https://doc.rust-lang.org/std/thread/index.html: "Module std::thread" (Rust 1.99.1, 2026-10-13).
10. https://doc.rust-lang.org/cargo/reference/profiles.html: "Profiles", The Cargo Book (no version on the section).
11. https://gafferongames.com/post/why_cant_i_send_udp_packets_from_a_browser/: Glenn Fiedler, "Why can't I send UDP
    packets from a browser?" (2017-02-26).
12. https://raw.githubusercontent.com/mas-bandwidth/netcode/main/STANDARD.md: netcode 1.02 standard (main).
13. https://www.rfc-editor.org/rfc/rfc7519.html: RFC 7519, "JSON Web Token (JWT)" (May 2015).
14. https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/tf/bot/tf_bot_manager.cpp:
    Team Fortress 2 bot manager (Source SDK 2013, master).
15. https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket: "WebSocket() constructor" (modified
    2026-06-22).
16. https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport: "WebTransport() constructor" (modified
    2026-10-12).
17. https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/: "WebTransport over HTTP/3",
    draft-ietf-webtrans-http3-16 (2026-07-06).
18. https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html: OWASP "Session Management Cheat
    Sheet" (no page date).
19. https://learn.microsoft.com/en-us/xbox/playfab/identity/player-identity/login/login-basics-best-practices: "Login
    basics and best Practices - PlayFab" (2025-02-20, updated 2025-10-31).
20. https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/ownership-changes: "Lobby ownership
    changes - PlayFab" (2023-02-28, updated 2025-09-10).
21. https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/create-a-lobby: "Create a lobby -
    PlayFab" (2021-10-25, updated 2026-03-24).

Tiers: official 18 (1-10, 13, 15-21); shipped game 3 (11, 12, 14); other 0. (Source 6 counts as official project docs.)

**Sources not read**
- https://developer.valvesoftware.com/wiki/Tf_bot_reevaluate_class_in_spawnroom and
  https://developer.valvesoftware.com/wiki/TFBot: HTTP 403 Forbidden on fetch (the TF2 source code was read instead).
- https://1-58-0.agones.dev/site/docs/integration-patterns/high-density-gameservers/ (and other agones.dev versions):
  outside the allow list (the same page was read from GitHub).
- https://docs.amazonaws.cn/en_us/gamelift/latest/developerguide/integration-server-sdk-unreal-ref-actions.html and
  other docs.amazonaws.cn pages: outside the allow list (docs.aws.amazon.com read instead).
- https://learn.microsoft.com/en-us/xbox/playfab/multiplayer/servers/billing-for-thunderhead: not fetched, fleet cost
  is R6's (search lead only).

## 7. Cross-unit notes (pointers, no claims)

- **WebTransport and WebSocket unit:** routing sessions by `:path` if many rooms share a process (F13); one port per
  room process (F18); the token in the first reliable message for both transports; QUIC connection migration in
  browsers (OQ8); per-process certificates if `serverCertificateHashes` is used.
- **Snapshots, prediction, interpolation unit:** a join or rejoin needs a full baseline snapshot and reset acks;
  `JoinAccepted` carries the server tick for clock sync.
- **Tick rate and bandwidth unit:** REC2's tick-p99 pass mark uses its tick period; rooms per host follow from its CPU
  budget.
- **Interest management unit:** a private-room spectator or caster (not in the brief) would need its own interest
  rule; no research done.
- **R1:** the sim must be instantiable per room with no globals (GW1, GE12); mode and rules file format, schema and hot
  reload (R1 data unit); seeded RNG per room.
- **R6:** the room supervisor, fleet scaling, regions, draining on deploys and VM retirement, per-room crash reports,
  the lobby's store, signing-key storage; the security baseline is deferred by the Advisor.
- **Security-auditor (R2 protocol review):** OQ2, rate limits, code brute force, host-command authority.
- **R5 / test-engineer:** the network simulator and soak runs behind REC3, REC4 and REC6.
- **Matchmaking beyond rooms** (public queues, skill): not in the brief; not researched.
- **game-designer / Mustafa:** OQ3-OQ7.
