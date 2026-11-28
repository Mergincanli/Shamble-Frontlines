// Test 1 (u8-brief.md rev 1.1 section 3): the two cloud-init files.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { statSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { OPS, heredoc, leaks, nftCheck, text, yamlLoad } from "./helpers.mjs";

const FILES = { rs: "cloud-init/rs.yaml", lg: "cloud-init/lg.yaml" };
// plan.md:1138-1147 as NPT-TRIM trims it (replan-units.md:152-153): one room user, no per-slot users.
const USERS = { rs: ["npt", "npt-build", "npt-host", "npt-room"], lg: ["npt", "npt-build", "npt-load"] };
// No extra group: the room reads the TLS key through its own group (setpriv clears supplementary groups).
const GROUPS = { rs: [], lg: [] };
// plan.md:1150-1156.
const SSHD_COMMON = [
  "PasswordAuthentication no",
  "KbdInteractiveAuthentication no",
  "PermitRootLogin no",
  "AuthenticationMethods publickey",
  "X11Forwarding no",
  "AllowAgentForwarding no",
  "MaxAuthTries 3",
  "LoginGraceTime 20",
];
const SSHD_ROLE = {
  rs: [
    "AllowUsers npt",
    "AllowTcpForwarding local",
    "PermitListen none",
    "AllowStreamLocalForwarding no",
    "PermitTunnel no",
    "GatewayPorts no",
    "PermitOpen none",
  ],
  lg: ["AllowUsers npt npt-load", "DisableForwarding yes"],
};
const WRAPPERS = { rs: "npt-ctl npt-cert npt-capture npt-userdata npt-s14", lg: "npt-ctl npt-userdata" };
const SUDOERS = { rs: "sudoers.d/npt", lg: "sudoers.d/npt-lg" };

function file(path, cfg) {
  return (cfg.write_files || []).find((f) => f.path === path);
}

for (const [role, rel] of Object.entries(FILES)) {
  const body = text(rel);
  const cfg = yamlLoad(rel);

  test(`${role}: starts #cloud-config, parses, under 32,768 bytes`, () => {
    assert.ok(body.startsWith("#cloud-config\n"));
    assert.equal(typeof cfg, "object");
    assert.ok(statSync(join(OPS, rel)).size < 32768, "Hetzner's 32 KiB user-data limit (plan.md:1180)");
  });

  test(`${role}: users and groups equal the trimmed list; cloud-init grants no sudo`, () => {
    const du = cfg.system_info.default_user;
    assert.equal(du.name, "npt");
    assert.equal(du.sudo, null);
    assert.equal(du.lock_passwd, true);
    const named = cfg.users.filter((u) => u !== "default");
    assert.ok(cfg.users.includes("default"), "npt is the default user, so the operator key reaches npt only");
    assert.deepEqual([du.name, ...named.map((u) => u.name)].sort(), USERS[role]);
    for (const u of named) {
      assert.ok(!("sudo" in u) || u.sudo === null, `${u.name}: no sudo from cloud-init`);
      assert.equal(u.lock_passwd, true, `${u.name}: locked password`);
    }
    for (const u of named.filter((x) => x.name === "npt-room" || x.name === "npt-host" || x.name === "npt-build")) {
      assert.equal(u.shell, "/usr/sbin/nologin", `${u.name}: no shell`);
    }
    assert.deepEqual(cfg.groups || [], GROUPS[role]);
    assert.equal(cfg.ssh_pwauth, false);
    assert.equal(cfg.disable_root, true);
    assert.deepEqual(cfg.ssh_genkeytypes, ["ed25519"]);
    assert.ok(!/sudoers\.d\/90-cloud-init-users/.test(body) || /rm -f \/etc\/sudoers\.d\/90-cloud-init-users/.test(body));
  });

  test(`${role}: the room and host users' fixed uids match the nft rules`, () => {
    const uid = Object.fromEntries(cfg.users.filter((u) => u !== "default").map((u) => [u.name, u.uid]));
    if (role === "rs") {
      assert.equal(uid["npt-room"], 2001);
      assert.equal(uid["npt-host"], 2002);
    } else {
      assert.equal(uid["npt-load"], 2004);
    }
  });

  test(`${role}: no address outside redact-check's allow-list, no secret pattern`, () => {
    assert.deepEqual(leaks(body), []);
  });

  test(`${role}: bootcmd's nftables text passes nft -c -f`, () => {
    const nft = heredoc(cfg.bootcmd.join("\n"), "NFT");
    assert.ok(nft, "an inline ruleset in bootcmd");
    assert.match(nft, /policy drop;/);
    assert.match(nft, /^table inet npt \{/m);
    assert.doesNotMatch(nft, /dport 80\b/, "no TCP 80 (the re-plan: DNS-01 by hand)");
    const r = nftCheck(nft);
    assert.ok(r.ok, r.err);
  });

  if (role === "rs") {
    test("rs: the wrappers' nft texts (quic off and on, peers, the ACME window) pass nft -c beside bootcmd's", () => {
      const nft = heredoc(cfg.bootcmd.join("\n"), "NFT");
      const lib = join(OPS, "sbin", "npt-lib.sh");
      const rules = execFileSync(
        "bash",
        ["-c", '. "$1"; printf "%s\\n%s\\n" "$NPT_QUIC_OFF_RULE" "$NPT_QUIC_ON_RULE"; npt_metrics_rule 192.0.2.10 198.51.100.7', "_", lib],
        { encoding: "utf8" },
      );
      assert.match(rules, /timeout 15m/, "Q-U8-3: the block times out after 15 minutes");
      const acme = "add element inet npt acme_v4 { 192.0.2.20 timeout 35m, 192.0.2.21 timeout 35m }\n";
      const r = nftCheck(`${nft}\n${rules}${acme}flush set inet npt acme_v4\n`);
      assert.ok(r.ok, r.err);
    });
  }

  test(`${role}: the sshd drop-ins hold every plan.md:1150-1156 key`, () => {
    const conf = ["/etc/ssh/sshd_config.d/00-npt.conf", "/etc/ssh/sshd_config.d/00-npt-open.conf"]
      .map((p) => file(p, cfg)?.content || "")
      .join("\n");
    for (const line of [...SSHD_COMMON, ...SSHD_ROLE[role]]) {
      assert.ok(conf.split("\n").includes(line), `missing: ${line}`);
    }
  });

  test(`${role}: names the code commit as 40 hex digits (the commit pair)`, () => {
    assert.match(file("/etc/npt/commit", cfg).content, /^[0-9a-f]{40}\n$/);
  });

  test(`${role}: toolchains pinned by SHA-256, never piped into a shell or included by URL`, () => {
    assert.doesNotMatch(body, /\|\s*(ba)?sh\b/);
    assert.doesNotMatch(body, /#include/);
    const downloads = body.match(/-o (rustup-init|node\.tar\.xz)/g) || [];
    assert.ok(downloads.length >= 1);
    for (const d of downloads) {
      const name = d.slice(3).replace(".", "\\.");
      assert.match(body, new RegExp(`[0-9a-f]{64}  ${name}" \\| sha256sum -c -`), `${d}: hash-checked`);
    }
    assert.match(body, /cargo fetch --locked/);
    assert.match(body, /cargo build --locked --offline/);
    if (role === "rs") {
      assert.match(body, /cargo install wasm-bindgen-cli --version 0\.2\.130 --locked/);
      assert.match(body, /npm_config_ignore_scripts=true npm ci --ignore-scripts/);
      assert.doesNotMatch(body, /\bnpx\b|npm exec|npm create|npm install/);
    }
  });

  test(`${role}: 2.7 root never writes or reads a user's home; bounded fetches; fingerprints before any fetch`, () => {
    const fb = file("/usr/local/sbin/npt-firstboot", cfg).content;
    const asRoot = fb.replace(/<<'BUILD'[^]*?\n\s*BUILD\n/, "<<'BUILD'\n");
    for (const l of asRoot.split("\n")) {
      if (/\b(install|chown|chmod|cat|tar -C)\b/.test(l) && /\/home\/|"\$NB|"\$T\/|"\$TB"|"\$D"/.test(l)) {
        assert.match(l, /runuser -u (npt|npt-build) -- /, `root touches a home path as itself (SEC-SB-2, -6): ${l.trim()}`);
      }
    }
    for (const l of fb.split("\n").filter((x) => /\bcurl\b/.test(x))) assert.match(l, /--max-time \d+ .*--speed-limit \d+ --speed-time \d+/, `INF-SB-2, INF-R2-4: ${l.trim()}`);
    for (const l of fb.split("\n").filter((x) => /\bgit\b.* fetch /.test(x))) {
      assert.match(l, /-c http\.lowSpeedLimit=\d+ -c http\.lowSpeedTime=\d+/, `INF-SB-2: ${l.trim()}`);
    }
    const fp = fb.indexOf("ssh-keygen -l -E sha256 -f");
    const fetches = [/\n\s*git\b[^\n]* fetch /, /\n\s*curl --/, /\n\s*unattended-upgrade /].map((r) => fb.search(r)).filter((i) => i >= 0);
    assert.ok(fp > 0 && fetches.every((i) => fp < i), "F12: the console fingerprint step comes before any fetch");
    assert.match(fb, /> \/etc\/issue\.d\/50-npt-hostkeys\.issue\n\s*agetty --reload/);
    const dir = fb.indexOf("\ninstall -d -o root -g root -m 0755 /etc/issue.d ");
    assert.ok(dir > 0 && dir < fb.indexOf("> /etc/issue.d/"), "SEC-R2-1: the folder is made before the write (24.04 has none)");
    if (role === "rs") {
      assert.ok(fb.indexOf("command -v dig tcpdump") > 0 && fb.indexOf("command -v dig tcpdump") < fb.indexOf("root-owned clone"), "INF-SB-8");
      const nft = heredoc(cfg.bootcmd.join("\n"), "NFT");
      const lo = nft.indexOf('oifname "lo" accept');
      for (const r of ["meta skuid 2001 ip daddr 127.0.0.53 drop", "meta skuid 2001 th dport 53 drop"]) {
        assert.ok(nft.includes(r) && nft.indexOf(r) < lo, `SEC-SB-10: ${r} before the loopback accept`);
      }
    }
  });

  test(`${role}: installs exactly its wrappers and its sudoers file, checked by visudo first`, () => {
    assert.match(body, new RegExp(`for w in ${WRAPPERS[role]}; do install -m 0755`));
    const s = SUDOERS[role].replace(".", "\\.");
    assert.match(body, new RegExp(`visudo -c -q -f "\\$O/sbin/${s}"\\n\\s*install -m 0440 "\\$O/sbin/${s}" /etc/sudoers\\.d/npt`));
  });
}

/** A systemd unit file as { section: { key: value } } (comments and blank lines skipped; a repeated key fails). */
function unit(rel) {
  const out = {};
  let sec = null;
  for (const raw of text(rel).split("\n")) {
    const l = raw.trim();
    if (!l || l.startsWith("#")) continue;
    const s = l.match(/^\[([A-Za-z]+)\]$/);
    if (s) {
      sec = out[s[1]] = {};
      continue;
    }
    const [k, ...v] = l.split("=");
    assert.ok(sec && !(k in sec), `${rel}: ${l}`);
    sec[k] = v.join("=");
  }
  return out;
}

test("rs: npt-host.path and npt-host.service as Q-U6C-7 rules them, and /etc/npt/host.json (fix pass item 1)", () => {
  const path = unit("systemd/npt-host.path");
  assert.deepEqual(Object.keys(path).sort(), ["Install", "Path", "Unit"]);
  assert.deepEqual(path.Path, { PathExists: "/var/lib/npt/tls/production.chain.pem" });
  assert.deepEqual(path.Install, { WantedBy: "multi-user.target" });
  const svc = unit("systemd/npt-host.service");
  assert.deepEqual(Object.keys(svc).sort(), ["Service", "Unit"], "no [Install]: only the path unit starts it");
  assert.deepEqual(svc.Service, {
    Type: "simple",
    User: "npt-host",
    Group: "npt-host",
    NoNewPrivileges: "yes",
    ExecStart: "/opt/npt/bin/npt-host serve /etc/npt/host.json",
    Restart: "no",
    // SEC-SB-9's subset, options a read-only server cannot trip (the rest at U6c's merge, from the host's needs).
    ProtectHome: "yes",
    PrivateTmp: "yes",
    CapabilityBoundingSet: "",
    RestrictNamespaces: "yes",
    LockPersonality: "yes",
    SystemCallArchitectures: "native",
  });
  // INF-SB-6: the start limit, not Restart=no, ends a flapping host; r1.sh refuses a failed path unit.
  assert.deepEqual([svc.Unit.StartLimitIntervalSec, svc.Unit.StartLimitBurst], ["300", "3"]);
  assert.match(text("runs/r1.sh"), /! systemctl is-failed --quiet npt-host\.path \|\| die "npt-host\.path failed/);
  assert.deepEqual(JSON.parse(text("systemd/host.json")), {
    bind: "0.0.0.0:31000",
    cert_chain: "/var/lib/npt/tls/production.chain.pem",
    cert_key: "/var/lib/npt/tls/production.key.pem",
    dist: "/var/lib/npt/dist",
    hash_template: "/var/lib/npt/hash/room-NN.json",
    host_keys: "/var/lib/npt/dist/ssh-host-keys.txt",
  });
  // Installed by root, 0644, from the root-owned clone, after the binaries and the host-key file, then enabled.
  const rs = text(FILES.rs);
  const lines = [
    'install -o root -g root -m 0644 "$O/systemd/host.json" /etc/npt/host.json',
    'install -o root -g root -m 0644 "$O/systemd/npt-host.path" "$O/systemd/npt-host.service" /etc/systemd/system/',
    "systemctl daemon-reload",
    "systemctl enable --now npt-host.path",
  ];
  let at = rs.indexOf("> /var/lib/npt/dist/ssh-host-keys.txt");
  assert.ok(at > rs.indexOf('install -m 0755 "$T/$b" "/opt/npt/bin/$b"'), "the host-key file after the binaries");
  for (const l of lines) {
    const i = rs.indexOf(`\n      ${l}\n`);
    assert.ok(i > at, `rs.yaml: ${l} (in order, after the binaries and the host-key file)`);
    at = i;
  }
  assert.ok(at < rs.indexOf('touch "$B/build.done"'));
  assert.doesNotMatch(rs, /npt-host\.service\b[^\n]*(enable|start)|systemctl (start|restart) npt-host/, "only the path unit starts the host");
});
