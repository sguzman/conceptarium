#!/usr/bin/env python3
"""Manage Conceptarium predicate presence and lazy concept capture."""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys
from typing import Any

import yaml


ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "registry" / "concepts.yml"
ONTOLOGY_STATES = {
    "unassessed",
    "unplaced",
    "roughly-classified",
    "domain-placed",
    "related",
    "deeply-integrated",
}


def load_registry() -> dict[str, Any]:
    data = yaml.safe_load(REGISTRY.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or not isinstance(data.get("concepts"), list):
        raise ValueError("registry/concepts.yml must contain a top-level concepts list")
    return data


def save_registry(data: dict[str, Any]) -> None:
    REGISTRY.write_text(
        yaml.safe_dump(data, sort_keys=False, allow_unicode=True, width=100),
        encoding="utf-8",
    )


def slugify(term: str) -> str:
    value = term.strip().casefold()
    value = value.replace("/", " ")
    value = re.sub(r"[^a-z0-9]+", "-", value)
    return value.strip("-")


def by_id(data: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        str(item["id"]): item
        for item in data["concepts"]
        if isinstance(item, dict) and item.get("id")
    }


def capture(args: argparse.Namespace) -> int:
    data = load_registry()
    concepts = data["concepts"]
    index = by_id(data)

    concept_id = args.id or slugify(args.term)
    if not concept_id:
        raise ValueError("could not derive a stable id; provide --id")

    if concept_id in index:
        existing = index[concept_id]
        print(
            f"{concept_id} already registered as {existing.get('term')!r} "
            f"({existing.get('materialization')})"
        )
        return 0

    record: dict[str, Any] = {
        "id": concept_id,
        "term": args.term.strip(),
        "presence": "registered",
        "materialization": "registry-only",
        "ontology_state": args.ontology_state,
        "registered_on": args.date,
    }

    if args.group:
        record["queue_group"] = args.group

    capture_meta: dict[str, Any] = {}
    if args.note:
        capture_meta["note"] = args.note
    if args.context:
        capture_meta["context"] = args.context
    if capture_meta:
        record["capture"] = capture_meta

    concepts.append(record)
    concepts.sort(key=lambda item: str(item.get("id", "")))
    save_registry(data)

    print(f"Registered {concept_id}: {args.term}")
    return 0


def materialize(args: argparse.Namespace) -> int:
    data = load_registry()
    index = by_id(data)

    if args.id not in index:
        raise ValueError(f"{args.id!r} is not registered")

    record = index[args.id]
    entry_path = Path(args.entry)
    if entry_path.is_absolute():
        raise ValueError("--entry must be repository-relative")

    record["materialization"] = "entry"
    record["entry"] = entry_path.as_posix()
    if record.get("ontology_state") == "unplaced":
        record["ontology_state"] = "unassessed"

    save_registry(data)
    print(f"Materialized {args.id} at {entry_path.as_posix()}")
    return 0


def queue(args: argparse.Namespace) -> int:
    data = load_registry()
    items = [
        item
        for item in data["concepts"]
        if isinstance(item, dict) and item.get("materialization") == "registry-only"
    ]

    if args.group:
        items = [item for item in items if item.get("queue_group") == args.group]

    current_group: str | None = None
    for item in sorted(
        items,
        key=lambda x: (str(x.get("queue_group", "")), str(x.get("term", "")).casefold()),
    ):
        group = str(item.get("queue_group") or "Ungrouped")
        if group != current_group:
            if current_group is not None:
                print()
            print(f"[{group}]")
            current_group = group
        print(f"- {item.get('term')} ({item.get('id')})")

    print(f"\n{len(items)} registry-only concept(s)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    p_capture = sub.add_parser(
        "capture",
        help="register a concept without requiring definition or ontology work",
    )
    p_capture.add_argument("term")
    p_capture.add_argument("--id", help="stable id; derived from term when omitted")
    p_capture.add_argument(
        "--date",
        default="unknown",
        help="capture date; default is unknown rather than invented",
    )
    p_capture.add_argument("--group", help="optional promotion-queue grouping")
    p_capture.add_argument("--note", help="short preservation note")
    p_capture.add_argument("--context", help="optional immediate context")
    p_capture.add_argument(
        "--ontology-state",
        default="unplaced",
        choices=sorted(ONTOLOGY_STATES),
    )
    p_capture.set_defaults(func=capture)

    p_materialize = sub.add_parser(
        "materialize",
        help="point an existing registry concept at a newly created canonical entry",
    )
    p_materialize.add_argument("id")
    p_materialize.add_argument("--entry", required=True)
    p_materialize.set_defaults(func=materialize)

    p_queue = sub.add_parser("queue", help="show registry-only concepts")
    p_queue.add_argument("--group")
    p_queue.set_defaults(func=queue)

    args = parser.parse_args()
    try:
        return int(args.func(args))
    except (OSError, ValueError, yaml.YAMLError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
