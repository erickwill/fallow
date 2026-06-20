#!/usr/bin/env bash

# Shared helpers for GitLab MR integration scripts.

# Track mktemp files so an EXIT trap cleans them up on signal or early exit.
_FALLOW_TMPS=()
trap 'rm -f "${_FALLOW_TMPS[@]:-}"' EXIT

FALLOW_RENDER_ARGS=()

prepare_fallow_render_args() {
  local format=$1
  [ -f fallow-analysis-args.sh ] || return 1
  # shellcheck disable=SC1091
  source fallow-analysis-args.sh
  FALLOW_RENDER_ARGS=("${FALLOW_ANALYSIS_ARGS[@]}")
  local replaced=false
  for i in "${!FALLOW_RENDER_ARGS[@]}"; do
    if [ "${FALLOW_RENDER_ARGS[$i]}" = "--format" ] && [ $((i + 1)) -lt "${#FALLOW_RENDER_ARGS[@]}" ]; then
      FALLOW_RENDER_ARGS[$((i + 1))]="$format"
      replaced=true
      break
    fi
  done
  if [ "$replaced" != "true" ]; then
    FALLOW_RENDER_ARGS+=(--format "$format")
  fi
  if [ -z "${FALLOW_DIFF_FILE:-}" ] && [ -n "${CI_MERGE_REQUEST_DIFF_BASE_SHA:-}" ]; then
    if git diff "${CI_MERGE_REQUEST_DIFF_BASE_SHA}..HEAD" > fallow-mr.diff 2>fallow-mr-diff-stderr.log; then
      export FALLOW_DIFF_FILE="$PWD/fallow-mr.diff"
    else
      echo "WARNING: Failed to fetch MR diff; diff filter disabled, reporting all findings"
      rm -f fallow-mr.diff
    fi
  fi
  export FALLOW_DIFF_FILTER="${FALLOW_DIFF_FILTER:-added}"
}

curl_retry() {
  local attempts="${FALLOW_API_RETRIES:-3}"
  local delay="${FALLOW_API_RETRY_DELAY:-2}"
  local attempt=1
  local err out
  err=$(mktemp)
  out=$(mktemp)
  while true; do
    if curl -sf "$@" >"$out" 2>"$err"; then
      cat "$out"
      rm -f "$err" "$out"
      return 0
    fi
    # Match the Rust `with_rate_limit_retry` decision: 429 + 502/503/504 are
    # transient and worth retrying; persistent 5xx (500, 501, 505) and all
    # other 4xx surface immediately. curl -sf emits stderr like
    # `curl: (22) The requested URL returned error: 502 Bad Gateway`, so we
    # match either the explicit code or the rate-limit / Retry-After hints.
    if [ "$attempt" -ge "$attempts" ] \
        || ! grep -Eqi 'error: (429|502|503|504)|rate limit|Retry-After' "$err"; then
      cat "$err" >&2
      rm -f "$err" "$out"
      return 1
    fi
    echo "WARNING: GitLab API rate limit response; retrying (${attempt}/${attempts})" >&2
    sleep "$delay"
    attempt=$((attempt + 1))
  done
}

# Walk the GitLab REST API's Link-header pagination, concatenating every page
# of a JSON array into a single combined array on stdout. Last positional arg
# is the initial URL; preceding args are passed to curl_retry verbatim. Without
# this, high-comment MRs can silently lose notes outside the first page and
# re-post duplicates on every run.
curl_paginate() {
  local args=("$@")
  local last=$(( ${#args[@]} - 1 ))
  local url="${args[$last]}"
  unset 'args[last]'
  local headers body
  headers=$(mktemp)
  body=$(mktemp)
  local combined='[]'
  while [ -n "$url" ]; do
    if ! curl_retry -D "$headers" "${args[@]}" "$url" > "$body"; then
      rm -f "$headers" "$body"
      return 1
    fi
    # Defensively skip non-array pages (e.g. an error envelope) so callers
    # degrade to "no existing notes seen" instead of crashing on jq shape
    # errors.
    combined=$(jq -s 'map(arrays) | add // []' <(printf '%s' "$combined") "$body")
    url=$(grep -i '^link:' "$headers" \
      | tr ',' '\n' \
      | sed -n 's/.*<\([^>]*\)>.*rel="next".*/\1/p' \
      | head -1)
  done
  rm -f "$headers" "$body"
  printf '%s' "$combined"
}
