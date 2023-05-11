#!/bin/bash
export PS4='\n\$ '
set -efuo pipefail

TARGETDIR="$(dirname "$(dirname "$(readlink -f "$0")")")/target"
export PATH="${TARGETDIR}/debug:$PATH"
mkdir -p "${TARGETDIR}/iomux-make-examples"
cd "${TARGETDIR}/iomux-make-examples"

set -x

iomux echo foo

FINDLOG="$(mktemp)"

iomux find /etc/ | tee "$FINDLOG"

grep '^[0-9]*>' "$FINDLOG" | head -1 | sed 's/>.*$//'
grep '^[0-9]* ' "$FINDLOG" | sed 's/^[0-9]*  //'
grep '^[0-9]*!' "$FINDLOG" | sed 's/^[0-9]*! //'
grep '^[0-9]*> exit' "$FINDLOG" | sed 's/^.*exit //'
