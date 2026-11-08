#!/usr/bin/env python3
# Tests of siocoutq-root-check.py's provenance line (u3c review F-J and C2R-10), with no root, no iptables and no probe:
# the script is copied into a scratch git repository at its own path under spikes/netcode-proof/ and imported from
# there, so its HERE and SPIKE are the scratch copy's. check.mjs's self step runs this file (check.test.mjs).
#   python3 -I -B spikes/netcode-proof/results/u3c-kernel-check/test_siocoutq_root_check.py

import importlib.util
import os
import pathlib
import shutil
import subprocess
import tempfile
import unittest
from unittest import mock

SCRIPT = pathlib.Path(__file__).resolve().parent / "siocoutq-root-check.py"
AT = pathlib.PurePosixPath("spikes/netcode-proof/results/u3c-kernel-check")
COMMITTED_LOG = AT / "siocoutq-root-check-20261108T005258Z.txt"


def git(repo, *args):
    """git in the scratch repository `repo` and nowhere else (u3c review C2b F2): every GIT_* variable is left out of
    its environment, since a git hook or `git rebase --exec` exports GIT_DIR, GIT_INDEX_FILE or GIT_WORK_TREE, and with
    them the fixture's commit landed in that other repository; hooks are off (core.hooksPath is an empty folder) and
    commits are not signed."""
    hooks = tempfile.mkdtemp(prefix="npt-no-hooks-")
    try:
        return subprocess.run(
            [
                "git",
                "-C",
                str(repo),
                "-c",
                f"core.hooksPath={hooks}",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "user.name=test",
                "-c",
                "user.email=test@example.invalid",
                *args,
            ],
            capture_output=True,
            text=True,
            check=True,
            env={k: v for k, v in os.environ.items() if not k.startswith("GIT_")},
        ).stdout.strip()
    finally:
        os.rmdir(hooks)


def scratch_repo(prefix):
    """A new git repository in a scratch folder, with one committed file, and its HEAD."""
    repo = pathlib.Path(tempfile.mkdtemp(prefix=prefix)).resolve()
    (repo / "keep.txt").write_text("kept\n")
    git(repo, "init", "-q")
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "other")
    return repo, git(repo, "rev-parse", "HEAD")


class Provenance(unittest.TestCase):
    def setUp(self):
        self.repo = pathlib.Path(tempfile.mkdtemp(prefix="npt-provenance-")).resolve()
        self.addCleanup(shutil.rmtree, self.repo, True)
        here = self.repo / AT
        here.mkdir(parents=True)
        shutil.copy(SCRIPT, here / SCRIPT.name)
        (self.repo / COMMITTED_LOG).write_text("# a committed root-run log\n")
        source = self.repo / "spikes/netcode-proof/crates/room/src/lib.rs"
        source.parent.mkdir(parents=True)
        source.write_text("pub fn f() {}\n")
        git(self.repo, "init", "-q")
        git(self.repo, "add", "-A")
        git(self.repo, "commit", "-q", "-m", "fixture")
        self.sha = git(self.repo, "rev-parse", "HEAD")
        spec = importlib.util.spec_from_file_location("root_check", here / SCRIPT.name)
        self.mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.mod)

    def write(self, path, text="x\n"):
        target = self.repo / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)

    def changed(self, *paths):
        return f"spike commit {self.sha} (with uncommitted changes: {', '.join(str(p) for p in paths)})"

    def test_a_clean_tree_names_the_commit_alone(self):
        self.assertEqual(self.mod.provenance(), f"spike commit {self.sha}")

    def test_its_own_new_logs_are_left_out(self):
        self.write(AT / "siocoutq-root-check-20261109T010203Z.txt")
        # A dry run's log (its stamp shortened here: the full name reads as a token to redact-check).
        self.write(AT / "siocoutq-root-check-dry-20261109.txt")
        self.assertEqual(self.mod.provenance(), f"spike commit {self.sha}")

    def test_an_edit_to_a_committed_log_is_listed(self):
        self.write(COMMITTED_LOG, "# edited after the run\n")
        self.assertEqual(self.mod.provenance(), self.changed(COMMITTED_LOG))

    def test_a_source_change_and_a_stray_file_are_listed(self):
        self.write("spikes/netcode-proof/crates/room/src/planted.rs")
        self.write(AT / "siocoutq-root-check.py.bak")
        self.assertEqual(
            self.mod.provenance(),
            self.changed("spikes/netcode-proof/crates/room/src/planted.rs", AT / "siocoutq-root-check.py.bak"),
        )

    def test_a_log_name_elsewhere_is_listed(self):
        self.write("spikes/netcode-proof/crates/room/siocoutq-root-check-20261109T010203Z.txt")
        self.assertEqual(self.mod.provenance(), self.changed("spikes/netcode-proof/crates/room/siocoutq-root-check-20261109T010203Z.txt"))

    def test_another_repository_in_the_environment_is_not_named(self):
        # Run from a git hook (GIT_DIR, GIT_WORK_TREE and GIT_INDEX_FILE exported for another repository), the script
        # still names its own commit and its own changes (u3c review C2b F2).
        other, other_sha = scratch_repo("npt-other-")
        self.addCleanup(shutil.rmtree, other, True)
        self.write("spikes/netcode-proof/crates/room/src/planted.rs")
        exported = {"GIT_DIR": str(other / ".git"), "GIT_WORK_TREE": str(other), "GIT_INDEX_FILE": str(other / ".git" / "index")}
        with mock.patch.dict(os.environ, exported):
            line = self.mod.provenance()
        self.assertNotIn(other_sha, line)
        self.assertEqual(line, self.changed("spikes/netcode-proof/crates/room/src/planted.rs"))


if __name__ == "__main__":
    unittest.main()
