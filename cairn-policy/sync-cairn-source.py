#!/usr/bin/env python3
"""Import/check Cairn's exact pinned Nickel policy input closure without changing JSON."""

import argparse
import hashlib
import os
import subprocess
import sys
import tempfile
from pathlib import Path

REVISION = "b6759012db4dab0158936e995b7bad39ac10465b"
BASE_BLOB = "57d1f8194930ceacde797af4b53f90ad1daeac18"
SOURCES = (
    "default.ncl",
    "contracts.ncl",
    "lifecycle-store-contracts.ncl",
    "nominal-evidence-identities.ncl",
)
POLICY = Path(__file__).resolve().parent

# Consumer-facing assurance-targets material. Cairn main 830b544 grants
# `contracts/**` and `cairn-policy/assurance-targets.ncl` as MIT OR Apache-2.0
# at every revision and for byte-for-byte copies (docs/licensing.md and
# cairn-policy/LICENSING.md, "Consumer contracts and assurance-targets policy
# offer").
ASSURANCE_REVISION = "830b5446bca69497d34ba65b22d8833927e3b5da"
ASSURANCE_SOURCES = (
    ("contracts/assurance-targets.ncl", "assurance-targets-contract.ncl"),
    ("cairn-policy/assurance-targets.ncl", "assurance-targets-policy.ncl"),
)
LICENSE_SOURCES = ("MIT.txt", "Apache-2.0.txt")
SUMS_PATH = POLICY / "SHA256SUMS"
SUMMED = (
    "upstream-source/default.ncl",
    "upstream-source/contracts.ncl",
    "upstream-source/lifecycle-store-contracts.ncl",
    "upstream-source/nominal-evidence-identities.ncl",
    "generated/cairn-policy.base.json",
    "assurance-targets-contract.ncl",
    "assurance-targets-policy.ncl",
)


def git(repo: Path, *args: str) -> bytes:
    return subprocess.check_output(("git", "-C", str(repo), *args))


def verify(repo: Path, relative: str, destination: Path, import_source: bool, revision: str = REVISION) -> None:
    original = git(repo, "show", f"{revision}:{relative}")
    expected_blob = git(repo, "rev-parse", f"{revision}:{relative}").decode().strip()
    actual_blob = subprocess.check_output(("git", "hash-object", "--stdin"), input=original).decode().strip()
    if actual_blob != expected_blob:
        raise ValueError(f"Git object mismatch: {relative}")
    if destination.is_symlink() or destination.parent.is_symlink():
        raise ValueError(f"refusing symlinked pinned source path: {destination}")
    if import_source and not destination.exists():
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(original)
    if not destination.is_file() or destination.is_symlink() or destination.read_bytes() != original:
        raise ValueError(f"missing or modified pinned source: {destination}")
    print(f"{relative}: {expected_blob}")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify_licenses(repo: Path, import_source: bool) -> None:
    for name in LICENSE_SOURCES:
        upstream = git(repo, "show", f"{ASSURANCE_REVISION}:LICENSES/{name}")
        destination = POLICY.parent / "LICENSES" / name
        if destination.is_symlink() or (destination.exists() and not destination.is_file()):
            raise ValueError(f"refusing non-regular license path: {destination}")
        if import_source and not destination.exists():
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(upstream)
        if not destination.is_file() or destination.read_bytes() != upstream:
            raise ValueError(f"missing or modified license text: {destination}")
        print(f"LICENSES/{name}: {sha256(destination)}")


def render_sums() -> str:
    lines = []
    for relative in SUMMED:
        path = POLICY / relative
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"missing or symlinked summed file: {path}")
        lines.append(f"{sha256(path)}  {relative}")
    return "\n".join(lines) + "\n"


def verify_sums(write: bool) -> None:
    rendered = render_sums()
    if write:
        SUMS_PATH.write_text(rendered)
    if not SUMS_PATH.is_file() or SUMS_PATH.read_text() != rendered:
        raise ValueError(f"{SUMS_PATH.name} differs from the vendored bytes")
    print(f"{SUMS_PATH.name}: {len(SUMMED)} entries")


def verify_assurance(repo: Path, import_source: bool) -> None:
    if git(repo, "rev-parse", f"{ASSURANCE_REVISION}^{{commit}}").decode().strip() != ASSURANCE_REVISION:
        raise ValueError("expected assurance-targets Cairn commit is unavailable")
    for relative, local in ASSURANCE_SOURCES:
        verify(repo, relative, POLICY / local, import_source, ASSURANCE_REVISION)
    verify_licenses(repo, import_source)


def self_test() -> None:
    with tempfile.TemporaryDirectory() as directory:
        repo = Path(directory)
        subprocess.check_call(("git", "-C", str(repo), "init", "--quiet"))
        subprocess.check_call(("git", "-C", str(repo), "config", "user.email", "test@example.invalid"))
        subprocess.check_call(("git", "-C", str(repo), "config", "user.name", "test"))
        (repo / "contracts").mkdir()
        (repo / "cairn-policy").mkdir()
        (repo / "LICENSES").mkdir()
        (repo / "contracts/assurance-targets.ncl").write_text("{ version = 1 }\n")
        (repo / "cairn-policy/assurance-targets.ncl").write_text("{ policy = 1 }\n")
        (repo / "LICENSES/MIT.txt").write_text("MIT text\n")
        (repo / "LICENSES/Apache-2.0.txt").write_text("Apache text\n")
        subprocess.check_call(("git", "-C", str(repo), "add", "."))
        subprocess.check_call(("git", "-C", str(repo), "commit", "--quiet", "-m", "fixture"))
        revision = subprocess.check_output(("git", "-C", str(repo), "rev-parse", "HEAD")).decode().strip()

        work = repo / "work"
        work.mkdir()
        destination = work / "assurance-targets-contract.ncl"

        def rejected(label: str, action) -> None:
            try:
                action()
            except (OSError, subprocess.CalledProcessError, ValueError):
                return
            raise AssertionError(f"negative case was accepted: {label}")

        verify(repo, "contracts/assurance-targets.ncl", destination, True, revision)
        destination.write_text("{ version = 2 }\n")
        rejected("edited byte", lambda: verify(repo, "contracts/assurance-targets.ncl", destination, False, revision))
        destination.unlink()
        rejected("missing file", lambda: verify(repo, "contracts/assurance-targets.ncl", destination, False, revision))
        destination.symlink_to(repo / "contracts/assurance-targets.ncl")
        rejected("symlink", lambda: verify(repo, "contracts/assurance-targets.ncl", destination, False, revision))
        rejected("missing upstream", lambda: verify(repo, "contracts/absent.ncl", destination, False, revision))
        print("sync-cairn-source self-test: edited byte, missing file, symlink and missing upstream refused")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path, help="Cairn checkout containing the pinned Git commits")
    parser.add_argument("--import", dest="import_source", action="store_true", help="copy missing pinned Nickel source files")
    parser.add_argument("--write-sums", action="store_true", help="rewrite cairn-policy/SHA256SUMS from the vendored bytes")
    parser.add_argument("--self-test", action="store_true", help="run the negative fixtures; no upstream checkout is read")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return
    if args.upstream is None:
        parser.error("--upstream is required unless --self-test is used")
    repo = args.upstream.resolve()
    if git(repo, "rev-parse", f"{REVISION}^{{commit}}").decode().strip() != REVISION:
        raise ValueError("expected pinned Cairn commit is unavailable")
    for name in SOURCES:
        verify(repo, f"cairn-policy/{name}", POLICY / "upstream-source" / name, args.import_source)
    upstream_json = git(repo, "show", f"{REVISION}:cairn-policy/generated/cairn-policy.json")
    local_json = (POLICY / "generated/cairn-policy.base.json").read_bytes()
    if upstream_json != local_json:
        raise ValueError("Trellis base projection differs from pinned Cairn output")
    blob = subprocess.check_output(("git", "hash-object", "--stdin"), input=local_json).decode().strip()
    if blob != BASE_BLOB:
        raise ValueError("pinned generated projection Git blob ID differs")
    license_bytes = git(repo, "show", f"{REVISION}:LICENSES/AGPL-3.0-or-later.txt")
    if (POLICY.parent / "LICENSES/AGPL-3.0-or-later.txt").read_bytes() != license_bytes:
        raise ValueError("upstream AGPL license text differs")
    print(f"generated/cairn-policy.base.json: {blob}; license text: exact pinned bytes")
    verify_assurance(repo, args.import_source)
    verify_sums(args.write_sums)


if __name__ == "__main__":
    try:
        main()
    except (OSError, subprocess.CalledProcessError, ValueError) as error:
        sys.exit(str(error))
