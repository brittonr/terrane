#!/usr/bin/env python3
"""Freeze Trellis's bounds producer job and its complete ordinary Nix tool graph.

Acquisition only: no build, proof, theorem admission or acceptance is performed.
The JSON derivation graph is Nix's existing format, not a new proof registry.
"""
import argparse
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

MAX_GRAPH_BYTES = 268_435_456
MAX_DERIVATIONS = 32_768
MAX_SOURCES = 16_384
MAX_SOURCE_BYTES = 34_359_738_368
STORE_HASH = re.compile(r"[0-9abcdfghijklmnpqrsvwxyz]{32}-[A-Za-z0-9+._?=-]{1,222}")
REVISION = re.compile(r"[0-9a-f]{40}")
ROOT_OWNER_INPUTS = ("-mantle-job.ncl", "-profile.json")
JOB_SCHEMA = "trellis-mantle-producer-job/v1"
JOB_OWNER = "Trellis"
REQUIRED_OWNER_FILES = [
    "nix/trellis-bounds-verus-job.nix",
    "proof-rails/verus-bounds/mantle-job.ncl",
    "tools/mantle-producer/export.py",
]


def command(argv, maximum):
    result = subprocess.run([str(value) for value in argv], capture_output=True, timeout=180)
    if result.returncode:
        raise RuntimeError(result.stderr[:1_048_576].decode(errors="replace"))
    if len(result.stdout) > maximum or len(result.stderr) > 1_048_576:
        raise ValueError("producer metadata command exceeds output bound")
    return result.stdout


def source_bytes(path, limit=MAX_SOURCE_BYTES):
    if path.is_symlink():
        return len(str(path.readlink()).encode())
    if path.is_file():
        return path.stat().st_size
    if path.is_dir():
        total = 0
        for child in path.iterdir():
            total += source_bytes(child, limit)
            if total > limit:
                raise ValueError("source acquisition byte bound exceeded")
        return total
    raise ValueError(f"unsupported source object: {path}")


def check_source_bytes(root, sources, limit=MAX_SOURCE_BYTES):
    if len(sources) > MAX_SOURCES:
        raise ValueError("source count bound exceeded")
    total = 0
    for source in sorted(sources):
        path = Path(source)
        if path.parent != Path("/nix/store"):
            raise ValueError("undeclared non-store source input")
        total += source_bytes(path, limit)
        if total > limit:
            raise ValueError("source acquisition aggregate byte bound exceeded")
    return total


def check_store_object(path, versioned):
    if not isinstance(path, str):
        raise ValueError("invalid producer store object identity")
    name = path if versioned else path.removeprefix("/nix/store/")
    if (not versioned and not path.startswith("/nix/store/")) or not STORE_HASH.fullmatch(name):
        raise ValueError("invalid producer store object identity")
    return "/nix/store/" + name


def producer_graph(graph, drv, source_root):
    """Inspect raw legacy/v4 references only; never rewrite or rehash as legacy."""
    versioned = "version" in graph or "derivations" in graph
    if versioned:
        if type(graph.get("version")) is not int or graph["version"] != 4 or not isinstance(graph.get("derivations"), dict):
            raise ValueError("unsupported producer graph schema version")
        nodes = graph["derivations"]
        root_key = drv.removeprefix("/nix/store/")
        source_key = source_root.removeprefix("/nix/store/")
    else:
        nodes, root_key, source_key = graph, drv, source_root
    if not nodes or len(nodes) > MAX_DERIVATIONS or root_key not in nodes:
        raise ValueError("producer graph root missing or graph exceeds bound")

    check_store_object(root_key if versioned else drv, versioned)
    check_store_object(source_key if versioned else source_root, versioned)
    sources = set()
    for path, node in nodes.items():
        if not isinstance(node, dict):
            raise ValueError("malformed producer derivation node")
        check_store_object(path, versioned)
        if not path.endswith(".drv"):
            raise ValueError("invalid producer derivation identity")
        if versioned:
            if type(node.get("version")) is not int or node["version"] != 4 or "inputDrvs" in node or "inputSrcs" in node:
                raise ValueError("unsupported or mixed producer node schema version")
            dependencies, srcs = node["inputs"]["drvs"], node["inputs"]["srcs"]
        else:
            if "version" in node or "inputs" in node:
                raise ValueError("mixed legacy and versioned producer node schema")
            dependencies, srcs = node["inputDrvs"], node["inputSrcs"]
        if not isinstance(dependencies, dict) or not isinstance(srcs, list):
            raise ValueError("malformed producer input collections")
        for dependency, selected in dependencies.items():
            check_store_object(dependency, versioned)
            if dependency not in nodes:
                raise ValueError("incomplete recursive producer graph")
            if versioned and (
                not isinstance(selected, dict)
                or selected.get("dynamicOutputs") != {}
                or not isinstance(selected.get("outputs"), list)
                or not all(isinstance(value, str) for value in selected["outputs"])
            ):
                raise ValueError("unsupported producer dynamic input or malformed output selection")
        sources.update(check_store_object(path, versioned) for path in srcs)
        if path == root_key:
            bound = [check_store_object(entry, versioned) for entry in srcs]
            for suffix in ROOT_OWNER_INPUTS:
                if not any(entry.endswith(suffix) for entry in bound):
                    raise ValueError("producer root does not bind immutable owner source")
    return sources, "nix-derivation-show-v4" if versioned else "nix-derivation-show-legacy"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--nix", type=Path)
    parser.add_argument("--nickel", type=Path)
    parser.add_argument("--flake", help="explicit immutable flake reference for this committed owner source")
    parser.add_argument("--revision", help="exact committed owner Git revision expected from the flake")
    parser.add_argument("--graph-format", choices=["nix-derivation-show-legacy", "nix-derivation-show-v4"],
                        help="consumer's required raw Nix wire format; never convert or downgrade")
    parser.add_argument("--out", type=Path)
    parser.add_argument("--self-test", action="store_true", help="run pure negative fixtures; performs no acquisition")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return
    for name in ["nix", "nickel", "flake", "revision", "graph_format", "out"]:
        if getattr(args, name) is None:
            parser.error(f"--{name.replace('_', '-')} is required unless --self-test is used")
    nix, nickel = (path.resolve(strict=True) for path in (args.nix, args.nickel))
    if args.out.exists():
        parser.error("--out must not exist")
    if not REVISION.fullmatch(args.revision):
        raise ValueError("--revision requires an exact immutable Git object ID")
    metadata = command([nix, "flake", "metadata", "--json", "--no-write-lock-file", args.flake], 1_048_576)
    meta = json.loads(metadata)
    locked_revision = meta.get("revision")
    if locked_revision != args.revision:
        raise ValueError("immutable flake revision differs from requested owner revision")
    archive = json.loads(command([nix, "flake", "archive", "--json", "--no-write-lock-file", args.flake], 16_777_216))
    locked_root = Path(archive["path"])
    if not locked_root.is_dir():
        raise ValueError("archived immutable flake source is unavailable")
    for required in REQUIRED_OWNER_FILES:
        if not (locked_root / required).is_file():
            raise ValueError(f"immutable owner source lacks {required}; recorded fixture pins cannot substitute")
    if (locked_root / "tools/mantle-producer/export.py").read_bytes() != Path(__file__).read_bytes():
        raise ValueError("exporter differs from selected immutable owner source")
    policy = locked_root / "proof-rails/verus-bounds/mantle-job.ncl"
    job_bytes = command([nickel, "export", "--format", "json", policy], 1_048_576)
    job = json.loads(job_bytes)
    if job["schema_version"] != JOB_SCHEMA or job["owner"] != JOB_OWNER:
        raise ValueError("unsupported owner job contract")
    attribute = job["package_attribute"]
    # Nix's locked URL includes the resolved revision, even if the input was a branch.
    target = f"{meta['url']}#{attribute}"
    drv = command([nix, "eval", "--raw", "--no-write-lock-file", target + ".drvPath"], 4096).decode().strip()
    graph_bytes = command([nix, "derivation", "show", "--recursive", drv], MAX_GRAPH_BYTES)
    graph = json.loads(graph_bytes)
    if not isinstance(graph, dict) or not graph or len(graph_bytes) > MAX_GRAPH_BYTES:
        raise ValueError("invalid or oversized recursive producer graph")
    sources, graph_format = producer_graph(graph, drv, str(locked_root))
    if graph_format != args.graph_format or graph_format not in job["tools"]["graph_formats"]:
        raise ValueError("exported graph format differs from explicit consumer contract")
    total = check_source_bytes(locked_root, sources)
    args.out.mkdir(parents=True)
    (args.out / "job.json").write_bytes(job_bytes)
    (args.out / "tool-closure.derivation-json.json").write_bytes(graph_bytes)
    (args.out / "flake-metadata.json").write_bytes(metadata)
    (args.out / "flake.lock").write_bytes((locked_root / "flake.lock").read_bytes())
    (args.out / "root-derivation").write_text(drv + "\n")
    (args.out / "source-revision").write_text(locked_revision + "\n")
    source_root = args.out / "source-payloads"
    source_root.mkdir()
    for source in sorted(sources):
        path = Path(source)
        destination = source_root / path.name
        if path.is_dir() and not path.is_symlink():
            shutil.copytree(path, destination, symlinks=True)
        elif path.is_symlink():
            destination.symlink_to(path.readlink())
        else:
            shutil.copy2(path, destination)
    print(json.dumps({"root_derivation": drv, "source_revision": locked_revision,
                      "graph_format": graph_format,
                      "derivations": len(graph["derivations"]) if graph_format == "nix-derivation-show-v4" else len(graph),
                      "source_objects": len(sources),
                      "source_bytes": total, "proof_executed": False}, indent=2))


def self_test():
    store = "/nix/store/" + "0" * 32
    source = "/nix/store/" + "1" * 32 + "-source"
    job = "/nix/store/" + "5" * 32 + "-mantle-job.ncl"
    derivation = "/nix/store/" + "6" * 32 + "-trellis-bounds-verus-job.nix"
    profile = "/nix/store/" + "7" * 32 + "-profile.json"
    drv = store + "-job.drv"
    drv_name = drv.removeprefix("/nix/store/")
    source_key = source.removeprefix("/nix/store/")
    node = {"inputDrvs": {}, "inputSrcs": [source, job, derivation, profile]}
    legacy = {drv: node}
    assert producer_graph(legacy, drv, source) == ({source, job, derivation, profile}, "nix-derivation-show-legacy")
    v4 = {"version": 4, "derivations": {drv_name: {"version": 4, "inputs": {"drvs": {}, "srcs": [source_key, job.removeprefix("/nix/store/"), derivation.removeprefix("/nix/store/"), profile.removeprefix("/nix/store/")]}}}}
    assert producer_graph(v4, drv, source) == ({source, job, derivation, profile}, "nix-derivation-show-v4")
    for name, graph in [
        ("missing root", {"/nix/store/" + "2" * 32 + "-x.drv": node}),
        ("incomplete closure", {drv: {"inputDrvs": {"/nix/store/" + "3" * 32 + "-dep.drv": ["out"]}, "inputSrcs": [source, job, derivation]}}),
        ("unsupported version", {"version": 3, "derivations": {}}),
        ("mixed schema", {drv: {"version": 4, "inputs": {"drvs": {}, "srcs": [source]}}}),
        ("dynamic inputs", {"version": 4, "derivations": {
            drv_name: {"version": 4, "inputs": {"drvs": {"4" * 32 + "-d.drv": {"dynamicOutputs": {"x": "y"}, "outputs": []}}, "srcs": [source_key, job.removeprefix("/nix/store/"), derivation.removeprefix("/nix/store/"), profile.removeprefix("/nix/store/")]}},
            "4" * 32 + "-d.drv": {"version": 4, "inputs": {"drvs": {}, "srcs": []}},
        }}),
    ]:
        try:
            producer_graph(graph, drv, source)
        except (ValueError, KeyError):
            pass
        else:
            raise AssertionError(f"malformed graph accepted: {name}")
    try:
        producer_graph({drv: {"inputDrvs": {}, "inputSrcs": [source, job]}}, drv, source)
    except ValueError:
        pass
    else:
        raise AssertionError("root without the derivation source was accepted")
    assert REVISION.fullmatch("b9bd540cfd291b58bf2cb81a4a016b8763ccf9e2")
    assert not REVISION.fullmatch("b9bd540")
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        (root / "a").write_bytes(b"abcd")
        (root / "b").write_bytes(b"efgh")
        assert source_bytes(root, 8) == 8
        try:
            source_bytes(root, 7)
        except ValueError:
            pass
        else:
            raise AssertionError("source byte bound was not enforced")
        with tempfile.TemporaryDirectory() as nested:
            try:
                check_source_bytes(root, [nested])
            except ValueError:
                pass
            else:
                raise AssertionError("undeclared non-store source input was accepted")
    print(json.dumps({"self_test": "passed", "proof_executed": False}))


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, ValueError, KeyError) as error:
        print(f"trellis-mantle-producer-export: {error}", file=sys.stderr)
        sys.exit(1)
