#!/usr/bin/env bash
# clone-count-check.sh: a cheap, deterministic first-pass filter for the
# defensive-cloning failure mode Modules 01 and 02's real dry runs both found
# (`runs/2026-07-05-module-01-relay-dry-run/`, `runs/2026-07-05-module-02-dry-run/`):
# a solution that clones a whole borrowed/owned collection "to be safe"
# passes `cargo test` and default `cargo clippy -- -D warnings` identically
# to one that doesn't. This does not replace Coachgremlin's conceptual read
# (Workflow step 3, `~/hekton/gremlins/coaching/coachgremlin.md`) - it's a
# pre-filter that runs before it: cheap, textual, and gameable by construction
# (see Limitations below). Trialed against both dry runs' real diffs before
# being adopted; see docs/decisions.md, 2026-07-05.
#
# What it counts: `.clone()` occurrences on added lines (`+`, not `+++` file
# headers) in a unified diff, restricted to a given path if one is passed.
# What it does NOT do: distinguish a load-bearing clone (a value that must
# outlive a borrow) from a defensive one (cloned "to feel safer"), notice a
# clone expressed without literally calling `.clone()` (e.g. `.to_owned()`,
# `.to_vec()`, a manual copy loop), or understand whether a clone count above
# the expected baseline is actually wrong for a *different, still-valid*
# solution shape. It flags for a human/Coachgremlin look; it does not fail
# a build on its own account (see Usage).
#
# Usage:
#   scripts/clone-count-check.sh <diff-file> <expected-max-clones> [path-filter]
#
#   <diff-file>            a unified diff (e.g. `git diff` output, or a saved
#                           runs/*/attempt-*/diff.patch)
#   <expected-max-clones>  the module's own declared baseline (its rubric
#                           should state this - e.g. Module 02's is 1, the
#                           one load-bearing `longest_gap_goal` clone)
#   [path-filter]          optional: only count clones in diff hunks whose
#                           file path contains this substring (e.g. `lib.rs`,
#                           to ignore an incidental clone in a test helper)
#
# Exit 0: clone count <= expected max (no flag).
# Exit 1: clone count > expected max (flag for Coachgremlin's conceptual read).
# Exit 2: usage error.
set -uo pipefail

if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <diff-file> <expected-max-clones> [path-filter]" >&2
  exit 2
fi

DIFF_FILE="$1"
EXPECTED_MAX="$2"
PATH_FILTER="${3:-}"

if [ ! -f "$DIFF_FILE" ]; then
  echo "clone-count-check: no such diff file: $DIFF_FILE" >&2
  exit 2
fi

if ! [[ "$EXPECTED_MAX" =~ ^[0-9]+$ ]]; then
  echo "clone-count-check: expected-max-clones must be a non-negative integer, got: $EXPECTED_MAX" >&2
  exit 2
fi

# Walk the diff, tracking which file's hunk we're currently inside (`+++ b/...`
# lines), and only count added lines (`+`, not `+++`) while that file matches
# the optional path filter.
count=0
current_file=""
in_matching_file=true
while IFS= read -r line; do
  if [[ "$line" == "+++ "* ]]; then
    current_file="${line#+++ }"
    current_file="${current_file#b/}"
    if [ -z "$PATH_FILTER" ] || [[ "$current_file" == *"$PATH_FILTER"* ]]; then
      in_matching_file=true
    else
      in_matching_file=false
    fi
    continue
  fi
  if [[ "$line" == "+"* ]] && [[ "$line" != "+++"* ]] && [ "$in_matching_file" = true ]; then
    # Count every occurrence on the line, not just whether one exists - a
    # single added line can call `.clone()` more than once.
    rest="$line"
    while [[ "$rest" == *".clone()"* ]]; do
      count=$((count + 1))
      rest="${rest#*.clone()}"
    done
  fi
done < "$DIFF_FILE"

echo "clone-count-check: $count clone() call(s) on added lines${PATH_FILTER:+ matching '$PATH_FILTER'} (expected max: $EXPECTED_MAX)"

if [ "$count" -gt "$EXPECTED_MAX" ]; then
  echo "FLAGGED: clone count exceeds this exercise's expected baseline - route to Coachgremlin's conceptual read before trusting a green cargo test/clippy run."
  exit 1
fi

echo "OK: within expected baseline."
exit 0
