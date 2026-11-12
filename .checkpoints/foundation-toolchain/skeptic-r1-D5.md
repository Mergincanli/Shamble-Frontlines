# Skeptic r1, D5 (SA-2 Low): setpriv without `--inh-caps=-all` / `--bounding-set=-all`; the cut prints but asserts none of its properties

Plan: (1) re-read toolchain.yml:60-88 at 4d78ad6 (the setpriv flags, what the cut prints and what it checks); (2) re-read
the CI log of PR job 131510257155 (:254-255) and the push job for what the cut prints; (3) re-read setpriv(1) (the quoted
advice, the `-all` code path) and capabilities(7) / prctl(2) (uid-change and no_new_privs rules) from the local man pages
and util-linux; (4) re-run as root in .gate/skeptic-D5/: the job's exact setpriv vs with the two flags added, reading
/proc/self/status; then the finding's "relies on" claims: a parent with non-empty inheritable and ambient sets, and a
root step shell (`--reuid=0`), each with and without the two flags; (5) judge the exploit sketch's two branches
(container job, self-hosted root runner); (6) judge severity and the fix (right, complete, what it breaks). Head under
review 4d78ad6; worktree HEAD 32386e1 differs from it only in .checkpoints/ (git diff --stat 4d78ad6 HEAD).

## Checks
- C1 toolchain.yml at 4d78ad6 (git show, cat -n): :68 `sudo -n unshare --net -- bash -ec 'ip link set lo up; exec "$@"'`,
  :69 `setpriv --reuid="$(id -u)" --regid=nogroup --clear-groups --no-new-privs --`, no `--inh-caps`, no
  `--bounding-set`: HOLDS. Inside, :74 `id; ip -o link; rustc --version` are printed and never compared. But :76-81 is
  an assertion of the cut's network property: curl to the pre-resolved github.com:443 must exit 7 or 28, exit 0 or any
  other code turns the step red. So "asserts none of its properties" overstates: the network property is asserted
  behaviourally (IPv4 TCP to one address); the privilege properties (uid, gid and groups, no_new_privs, capability sets)
  and "only lo" are not. The header comment :4-5 claims two privilege properties: "no docker group and no_new_privs".
- C2 CI log, PR job 131510257155 (saved .gate/skeptic-D5/job-131510257155.log; image ubuntu-24.04 20261101.360.1,
  Ubuntu 24.04.5): :254 `uid=1001(runner) gid=65534(nogroup) groups=65534(nogroup)`, :255 one line, `lo` only, :259
  `the network is cut: no TCP connection to 140.82.114.4:443 (curl exit 7)`. No CapXxx or NoNewPrivs line is printed
  (the SA says so too). HOLDS: printed, not compared; capability sets and no_new_privs are not even printed.
- C3 local tools: root, util-linux 2.39.3-9ubuntu6.6 and libcap-ng 0.8.4 (Ubuntu 24.04's packages, the runner's OS),
  sudo 1.9.15p5, kernel cap_last_cap 40. This setpriv binary has no "libcap-ng is too old" exit (strings: only
  "cap %d: libcap-ng is broken"): `-all` runs to the kernel's cap_last_cap.
- C4 experiments, .gate/skeptic-D5/caps-test.sh (+ inner.sh), output caps-test.out, rc 0, every case exit 0:
  E1 job flags: uid 1001, gid/groups 65534, CapInh/Prm/Eff/Amb 0, CapBnd 000001fffeffffff (parent's), NoNewPrivs 1,
  ifaces lo (= the SA's test). E2 + `--inh-caps=-all --bounding-set=-all`: the same with CapBnd 0, exit 0 (= the SA).
  E3 "relies on sudo's root having empty inheritable and ambient sets": a parent root with CapInh = CapAmb = 0x201000
  (net_admin, sys_admin), then the job's flags: CapInh 0x201000 (kept), CapAmb 0 (cleared by the 0 -> 1001 uid
  change), Prm/Eff 0, and a binary with file caps `cap_net_admin,cap_sys_admin=ei` gets Prm/Eff 0. Control without
  --no-new-privs: the same binary gets 0x201000. So the result does NOT rely on those sets: no_new_privs makes a
  non-empty inheritable set inert and the uid change clears ambient; only the unprinted CapInh number would differ.
  With the two flags: CapInh 0 as well. DOES NOT HOLD as stated (the inheritable/ambient part).
  E4 a root step shell (`--reuid=0`, the other flags unchanged): uid 0, CapPrm/Eff = 000001fffeffffff, NoNewPrivs 1,
  ifaces lo, and nsenter into the pre-cut netns succeeds (sees lo ifb0 ifb1 eth0): HOLDS (the mechanism).
  E5 a root step shell + the two flags: caps all 0, nsenter refused (EACCES on /proc/<pid>/ns/net), but a root-owned
  660 stand-in for docker.sock stays writable (owner rights need no capability): the flags alone do not contain uid 0;
  only the fix's "id -u is not 0" assertion closes that case. E6 the job's flags (uid 1001): nsenter refused, the
  stand-in socket not writable (the groups are cleared).
- C5 sources (saved under .gate/skeptic-D5/src/): setpriv.1.adoc at util-linux v2.39.3 :71-73 says, word for word,
  "Setting a _uid_ or _gid_ does not change capabilities, although the exec call at the end might change
  capabilities. This means that, if you are root, you probably want to do something like: *setpriv --reuid=1000
  --regid=1000 --inh-caps=-all*", and :59 (--no-new-privs) "the set-user-ID and set-group-ID bits as well as file
  capabilities will be disabled": the SA's quotes HOLD. setpriv.c v2.39.3 :994-1059: NNP first, KEEPCAPS + CAP_SETPCAP
  kept across setresuid, then the bounding set, then the inheritable set, then execvp; do_caps :530-535 loops `-all`
  to the kernel's cap_last_cap() (no "too old" exit), so the flags cannot fail on a kernel with more capabilities.
  capabilities(7) (man7.org): a 0 -> all-nonzero uid change clears "the permitted, effective, and ambient" sets;
  "P'(ambient) = (file is privileged) ? 0 : P(ambient)", a privileged file being one with capabilities or the
  set-user-ID bit: sudo's root therefore always starts with an empty ambient set, and only inheritable can carry
  over; for a uid-0 exec "P'(permitted) = P(inheritable) | P(bounding)": why E4 keeps every capability and E5 none.
- C6 the exploit sketch's two branches. Container job: Docker's default capabilities (docs.docker.com, containers/run:
  AUDIT_WRITE, CHOWN, DAC_OVERRIDE, FOWNER, FSETID, KILL, MKNOD, NET_BIND_SERVICE, NET_RAW, SETFCAP, SETGID, SETPCAP,
  SETUID, SYS_CHROOT) exclude SYS_ADMIN, and its default seccomp profile lists `unshare` "Deny cloning new namespaces
  ... Also gated by CAP_SYS_ADMIN" (security/seccomp): `sudo -n unshare --net` fails, the step goes red (fail closed),
  and most images have no sudo at all. Green-but-uncut needs `container.options: --privileged` (GitHub's syntax page
  forbids only --network and --entrypoint) plus sudo in the image. Self-hosted: actions/runner config.sh :6 and
  run-helper.sh.template :5 refuse uid 0 unless RUNNER_ALLOW_RUNASROOT is set. So "a container job" overstates: both
  branches need a deliberate opt-in and a change to `runs-on`/`container` (a CI edit that asks him); the mechanism
  itself (uid 0 keeps every capability and re-enters the old netns, the probe still passes) HOLDS (E4).
- C7 the plan the cut was approved on (plan.md at HEAD): 4.2 :344-348 "Inside the cut the job prints `id` (no docker
  group), `ip -o link` (`lo` up; any other device printed and noted) ... it judges only the TCP probe": printing
  without judging was the approved design, not an oversight. 4.3 :375-377 and N6 :746: "fallback tunnel devices can
  appear in a new namespace when their kernel modules are loaded (the fold check's critic, N6); the job does not rest
  on it" (critic G6: sit0/tunl0/ip6tnl0). Kernel doc (docs.kernel.org sysctl/net, fb_tunnels_only_for_init_net):
  "value = 0; respective fallback tunnels are created when module is loaded in every net namespaces"; local value 0;
  not reproducible here (no loadable modules). 4.4 :388-389: "the cleared groups and `no_new_privs` close the two easy
  ways out (docker, sudo)"; N2 :741 "the runner's docker group would reach the Docker socket from inside the cut".
