#!/bin/sh
# truthcheck.sh — pin repo copy to Live Tennis API ground truth.
# Fails on stale quota numbers, wrong URLs and forbidden identifiers.
# CHANGELOG.md (history entries) and this script are exempt.
set -u

EXCLUDE=':!CHANGELOG.md :!scripts/truthcheck.sh'
fail=0

forbid() {
  # shellcheck disable=SC2086
  if git grep -inE "$1" -- . $EXCLUDE; then
    echo "FORBIDDEN: $2" >&2
    fail=1
  fi
}

forbid '(100[,.]?000|100k)[^0-9]{0,40}(/day|per day|daily)' 'stale 100k/day quota'
forbid '(/day|per day|daily)[^0-9]{0,40}(100[,.]?000|100k)' 'stale 100k/day quota'
forbid 'free[^0-9]{0,30}(1,?000|1k)[^0-9]{0,20}(/day|per day|requests/day|daily)' 'stale free 1,000/day quota'
forbid 'livetennisapi\.com/docs' 'wrong docs URL (use docs.livetennisapi.com)'
forbid 'bensynapse' 'personal handle in repo metadata'
forbid 'midnight UTC' 'wrong quota-reset copy'

# If the repo states quotas at all, the current FREE figure and the docs
# domain must both be present somewhere.
# shellcheck disable=SC2086
if git grep -qiE '(requests?/day|per day|/day)' -- . $EXCLUDE; then
  # shellcheck disable=SC2086
  if ! git grep -qE '100( requests)?/day' -- . $EXCLUDE; then
    echo "MISSING: FREE quota string '100/day' (or '100 requests/day')" >&2
    fail=1
  fi
  # shellcheck disable=SC2086
  if ! git grep -q 'docs\.livetennisapi\.com' -- . $EXCLUDE; then
    echo "MISSING: docs.livetennisapi.com link" >&2
    fail=1
  fi
fi

if [ "$fail" -eq 0 ]; then
  echo "truthcheck OK"
fi
exit "$fail"
