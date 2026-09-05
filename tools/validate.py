#!/usr/bin/env python3
"""Validate Conceptarium canonical entries.

Structural problems fail validation.
Relation-ontology expansion and dangling targets are warnings during migration.
Use --strict to make warnings fail as well.
"""

from __future__ import annotations

import argparse
from collections import defaultdict
from pathlib import Path
import sys
from typing import Any

import yaml


ROOT = Path(__file__).resolve().parents[1]
ENTRIES = ROOT / "entries"

ALLOWED_TYPES = {
    "concept",
    "distinction",
    "mechanism",
    "framework",
    "phrase",
    "failure-mode",
    "question",
    "method",
    "principle",
    "metaphor",
}

ALLOWED_STATUSES = {
    "seed",
    "provisional",
    "canonical",
    "contested",
    "deprecated",
    "archived",
}

ALLOWED_AUTHORSHIP = {
    "user",
    "assistant",
    "joint",
    "pre-existing",
    "unknown",
}

ALLOWED_CERTAINTY = {
    "exact",
    "approximate",
    "reconstructed",
    "unknown",
}

PREFERRED_RELATIONS = {
    # identity / evolution
    "alias-of",
    "refines",
    "refined-by",
    "supersedes",
    "superseded-by",
    "descends-from",
    "changes",
    "changed-by",
    # structure / taxonomy
    "part-of",
    "contains",
    "specializes",
    "generalizes",
    "instance-of",
    "contains-instance",
    # contrast
    "contrasts-with",
    "distinguishes-from",
    "overlaps-with",
    # causation / dynamics
    "causes",
    "caused-by",
    "produces",
    "produced-by",
    "enables",
    "enabled-by",
    "constrains",
    "constrained-by",
    "intensifies",
    "intensified-by",
    "relieves",
    "relieved-by",
    "threatens",
    "threatened-by",
    "prevents",
    "prevented-by",
    "stabilizes",
    "stabilized-by",
    # dependency / implementation
    "requires",
    "required-by",
    "uses",
    "used-by",
    "implements",
    "implemented-by",
    "operationalizes",
    "realizes",
    "realized-by",
    # epistemic / explanatory
    "explains",
    "predicts",
    "tests",
    "tested-by",
    "evaluates",
    "exemplifies",
    "motivates",
    "motivated-by",
    # representation / generation
    "translates",
    "expresses",
    "expressed-by",
    "generates",
    "generated-by",
    "built-from",
    # currently useful social/institutional verbs
    "incorporates",
    "characterizes",
    "characterized-by",
    "associated-with",
    "supports",
    "supported-by",
    "threatens",
    "prioritizes",
    "shapes",
    # established local vocabulary
    "solves",
    "solved-by",
    "pays",
    "paid-by",
    "compressed-by",
    "compressed-form-of",
    "counteracts",
    "counters",
    "moves-from",
    "moves-toward",
    "measured-by",
    "exposed-by",
    "penalizes",
    "populated-by",
    "complements",
    "acts-through",
    "communicates-through",
    "depends-on",
    "analogous-to",
    "generated-by",
}

BAD_FRAGMENT_RELATIONS = {"from", "into", "between", "to", "with"}


def frontmatter(path: Path) -> tuple[dict[str, Any], str]:
    raw = path.read_text(encoding="utf-8")
    lines = raw.splitlines()

    if not lines or lines[0].strip() != "---":
        raise ValueError("file does not begin with YAML frontmatter delimiter")

    try:
        end = next(i for i in range(1, len(lines)) if lines[i].strip() == "---")
    except StopIteration as exc:
        raise ValueError("frontmatter has no closing delimiter") from exc

    meta_raw = "\n".join(lines[1:end])
    body = "\n".join(lines[end + 1 :])
    meta = yaml.safe_load(meta_raw)

    if not isinstance(meta, dict):
        raise ValueError("frontmatter must parse to a mapping")

    return meta, body


def require_string(meta: dict[str, Any], key: str, errors: list[str], label: str) -> str | None:
    value = meta.get(key)
    if not isinstance(value, str) or not value.strip():
        errors.append(f"{label}: {key!r} must be a non-empty string")
        return None
    return value.strip()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--strict",
        action="store_true",
        help="treat warnings (including dangling/experimental relations) as failures",
    )
    args = parser.parse_args()

    errors: list[str] = []
    warnings: list[str] = []
    parsed: dict[str, tuple[Path, dict[str, Any], str]] = {}
    terms: dict[str, str] = {}
    alias_owners: defaultdict[str, set[str]] = defaultdict(set)

    paths = sorted(p for p in ENTRIES.glob("*.md") if p.name != "_template.md")

    if not paths:
        errors.append("no canonical entries found")

    for path in paths:
        label = path.relative_to(ROOT).as_posix()
        try:
            meta, body = frontmatter(path)
        except (OSError, ValueError, yaml.YAMLError) as exc:
            errors.append(f"{label}: {exc}")
            continue

        entry_id = require_string(meta, "id", errors, label)
        term = require_string(meta, "term", errors, label)
        require_string(meta, "gloss", errors, label)

        if entry_id:
            if entry_id != path.stem:
                errors.append(
                    f"{label}: id {entry_id!r} must match filename stem {path.stem!r}"
                )
            if entry_id in parsed:
                errors.append(f"{label}: duplicate id {entry_id!r}")
            parsed[entry_id] = (path, meta, body)

        entry_type = meta.get("type")
        if entry_type not in ALLOWED_TYPES:
            errors.append(f"{label}: invalid type {entry_type!r}")

        status = meta.get("status")
        if status not in ALLOWED_STATUSES:
            errors.append(f"{label}: invalid status {status!r}")

        domains = meta.get("domains")
        if not isinstance(domains, list) or not all(
            isinstance(x, str) and x.strip() for x in domains
        ):
            errors.append(f"{label}: domains must be a list of non-empty strings")

        aliases = meta.get("aliases")
        if not isinstance(aliases, list) or not all(
            isinstance(x, str) and x.strip() for x in aliases
        ):
            errors.append(f"{label}: aliases must be a list of non-empty strings")
            aliases = []

        origin = meta.get("origin")
        if not isinstance(origin, dict):
            errors.append(f"{label}: origin must be a mapping")
        else:
            if origin.get("authorship") not in ALLOWED_AUTHORSHIP:
                errors.append(
                    f"{label}: invalid origin.authorship {origin.get('authorship')!r}"
                )
            if origin.get("certainty") not in ALLOWED_CERTAINTY:
                errors.append(
                    f"{label}: invalid origin.certainty {origin.get('certainty')!r}"
                )
            if "date" not in origin:
                errors.append(f"{label}: origin.date is required")

        relations = meta.get("relations")
        if not isinstance(relations, list):
            errors.append(f"{label}: relations must be a list")
        else:
            for idx, relation in enumerate(relations):
                rel_label = f"{label}: relation[{idx}]"
                if not isinstance(relation, dict):
                    errors.append(f"{rel_label} must be a mapping")
                    continue
                rel_type = relation.get("type")
                target = relation.get("target")
                if not isinstance(rel_type, str) or not rel_type.strip():
                    errors.append(f"{rel_label}.type must be a non-empty string")
                else:
                    rel_type = rel_type.strip()
                    if rel_type in BAD_FRAGMENT_RELATIONS:
                        warnings.append(
                            f"{rel_label}: {rel_type!r} is a grammatical fragment; "
                            "rewrite the relation so the edge stands alone"
                        )
                    elif rel_type not in PREFERRED_RELATIONS:
                        warnings.append(
                            f"{rel_label}: experimental relation type {rel_type!r}; "
                            "document it in docs/RELATIONS.md if intentional"
                        )
                if not isinstance(target, str) or not target.strip():
                    errors.append(f"{rel_label}.target must be a non-empty string")

        if term:
            key = term.casefold()
            if key in terms and terms[key] != entry_id:
                errors.append(
                    f"{label}: term {term!r} duplicates canonical term owned by {terms[key]!r}"
                )
            elif entry_id:
                terms[key] = entry_id
                alias_owners[key].add(entry_id)

        if entry_id:
            for alias in aliases:
                alias_owners[alias.casefold()].add(entry_id)

        if status == "canonical":
            for heading in ("## Problem pressure", "## Provenance", "## Open questions"):
                if heading not in body:
                    warnings.append(f"{label}: canonical entry is missing {heading!r}")

    ids = set(parsed)

    for entry_id, (path, meta, _) in parsed.items():
        label = path.relative_to(ROOT).as_posix()
        for idx, relation in enumerate(meta.get("relations", [])):
            if not isinstance(relation, dict):
                continue
            target = relation.get("target")
            if isinstance(target, str) and target and target not in ids:
                warnings.append(
                    f"{label}: relation[{idx}] targets missing entry {target!r}"
                )

    for text, owners in sorted(alias_owners.items()):
        if len(owners) > 1:
            warnings.append(
                f"alias/term collision {text!r} resolves to multiple entries: "
                + ", ".join(sorted(owners))
            )

    for message in errors:
        print(f"ERROR: {message}", file=sys.stderr)
    for message in warnings:
        print(f"WARNING: {message}", file=sys.stderr)

    print(
        f"Validated {len(parsed)} entries: "
        f"{len(errors)} error(s), {len(warnings)} warning(s)."
    )

    if errors or (args.strict and warnings):
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
