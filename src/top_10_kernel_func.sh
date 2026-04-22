#!/bin/bash -x

# SPDX-License-Identifier: Apache-2.0

CLI_BIN="/home/fge/Source/lhpnc-play/target/debug/gcli"
SAMPLE_RATE=997

# 1. Allow kernel symbol resolution (required to map addresses to function names)
sysctl -w kernel.kptr_restrict=0 > /dev/null 2>&1

# 2. Record kernel CPU samples
perf record \
    --call-graph fp \
    -g \
    -F $SAMPLE_RATE \
    -o /tmp/kern_prof.perf.data \
    -- $CLI_BIN

# 3. Extract and display top 10 kernel functions by time consumed
echo -e "\nTop 10 Kernel Functions by Time Consumed:"
perf report \
    -i /tmp/kern_prof.perf.data \
    --no-children \
    -n --sort symbol \
    --dsos '[kernel.kallsyms]' | \
awk '
    /^#[[:space:]]/ { next }
    /^[[:space:]]+[0-9]/ { print; if (++c >= 10) exit }
'
