#!/usr/bin/env bash
# Builds the Calendar test page and drives it in Chromium (BASTION-539).
#   PLAYWRIGHT_MODULE=/path/to/node_modules/playwright tests/browser/run.sh
# Optional: BULMA_CSS, FONTAWESOME_CSS, SCREENSHOT_DIR (see calendar.test.mjs).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
(cd "$here/calendar_page" && trunk build --dist dist >/dev/null)
CALENDAR_PAGE_DIST="$here/calendar_page/dist" node --test "$here/calendar.test.mjs"
