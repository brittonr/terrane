#!/usr/bin/env python3
"""Materialize and run each convergence negative record against a temp copy."""
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

def hardlink_tree(src, dst):
    for path in sorted(src.rglob("*")):
        rel = path.relative_to(src)
        target = dst / rel
        if path.is_dir():
            target.mkdir(parents=True, exist_ok=True)
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            os.link(path, target)

def rewrite(path, data):
    path.unlink()
    path.write_bytes(data)

def apply(case, root, mut, work):
    if mut["op"] == "json-set":
        path = case / mut["file"]
        value = json.loads(path.read_text())
        keys = [k for k in mut["pointer"].split("/") if k]
        node = value
        for key in keys[:-1]:
            node = node[int(key)] if isinstance(node, list) else node[key]
        last = keys[-1]
        if isinstance(node, list):
            node[int(last)] = mut["value"]
        else:
            node[last] = mut["value"]
        rewrite(path, (json.dumps(value, indent=2) + "\n").encode())
        return root
    if mut["op"] in ("case-set", "case-remove"):
        path = case / "case.json"
        value = json.loads(path.read_text())
        if mut["op"] == "case-set":
            keys = [k for k in mut["pointer"].split("/") if k]
            node = value
            for key in keys[:-1]:
                node = node[int(key)] if isinstance(node, list) else node[key]
            node[keys[-1]] = mut["value"]
        else:
            for pointer in mut["keys"]:
                keys = [k for k in pointer.split("/") if k]
                node = value
                for key in keys[:-1]:
                    node = node[int(key)] if isinstance(node, list) else node[key]
                node.pop(keys[-1], None)
        rewrite(path, (json.dumps(value, indent=2) + "\n").encode())
        return root
    if mut["op"] == "replace-bytes":
        path = case / mut["file"]
        data = path.read_bytes()
        if mut["find"].encode() not in data:
            raise SystemExit(f"{mut['file']}: mutation text absent")
        rewrite(path, data.replace(mut["find"].encode(), mut["replace"].encode(), 1))
        return root
    if mut["op"] == "root-replace-bytes":
        staged = work / "root"
        shutil.copytree(root / ".cairn" / "specs", staged / ".cairn" / "specs", copy_function=shutil.copy2)
        shutil.copytree(root / "cairn-policy", staged / "cairn-policy", copy_function=shutil.copy2)
        path = staged / mut["file"]
        data = path.read_bytes()
        if mut["find"].encode() not in data:
            raise SystemExit(f"{mut['file']}: mutation text absent")
        rewrite(path, data.replace(mut["find"].encode(), mut["replace"].encode(), 1))
        return staged
    raise SystemExit(f"unsupported mutation {mut['op']}")

def main():
    cairn, case, root = (Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve(), Path(sys.argv[3]).resolve())
    records = sorted((case / "negatives").glob("*.json"))
    failed = 0
    for record_path in records:
        record = json.loads(record_path.read_text())
        work = Path(tempfile.mkdtemp(dir=case.parent))
        try:
            staged = work / "case"
            hardlink_tree(case, staged)
            used_root = apply(staged, root, record["mutation"], work)
            result = subprocess.run(
                [str(cairn), "convergence", "inspect", "--case", str(staged / "case.json"), "--root", str(used_root)],
                capture_output=True, text=True)
            expected = record["expected"]
            problems = []
            if result.returncode != expected["exit"]:
                problems.append(f"exit {result.returncode} != {expected['exit']}")
            try:
                receipt = json.loads(result.stdout)
            except json.JSONDecodeError:
                receipt = None
            if receipt is None:
                problems.append("no JSON receipt")
            else:
                issues = [i["code"] for i in receipt.get("issues", [])]
                if expected["row_state"] is None:
                    pass
                else:
                    row = next((r for r in receipt.get("rows", []) if r.get("requirement_id") == "bounds.clamp"), None)
                    if row is None:
                        problems.append("bounds.clamp row missing")
                    else:
                        if row.get("state") != expected["row_state"]:
                            problems.append(f"state {row.get('state')} != {expected['row_state']}")
                        issues = row.get("issues", [])
                for code in expected["issues"]:
                    if code not in issues:
                        problems.append(f"missing issue {code}")
            if problems:
                failed += 1
                print(f"FAIL {record_path.name}: {'; '.join(problems)}")
            else:
                print(f"ok   {record_path.name}")
        finally:
            shutil.rmtree(work, ignore_errors=True)
    print(f"{len(records) - failed}/{len(records)} convergence negatives behaved as specified")
    sys.exit(1 if failed else 0)

if __name__ == "__main__":
    main()
