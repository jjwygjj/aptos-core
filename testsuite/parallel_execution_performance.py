#!/usr/bin/env python3

# Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

import subprocess
import re
import platform

# Set the tps and speedup ratio threshold for block size 1k, 10k and 50k
THRESHOLDS = {
    "1k_8": 11000,
    "1k_16": 13000,
    "1k_32": 13000,
    "10k_8": 22000,
    "10k_16": 32000,
    "10k_32": 43000,
    "50k_8": 49000,
    "50k_16": 64000,
    "50k_32": 86000,
}

SPEEDUPS = {
    "1k_8": 3,
    "1k_16": 4,
    "1k_32": 4,
    "10k_8": 5,
    "10k_16": 8,
    "10k_32": 10,
    "50k_8": 5,
    "50k_16": 9,
    "50k_32": 10,
}

THRESHOLDS_NOISE = 0.20
SPEEDUPS_NOISE_BELOW = 1
SPEEDUPS_NOISE_ABOVE = 2

THREADS = [8, 16, 32]
BLOCK_SIZES = ["1k", "10k", "50k"]
target_directory = "aptos-move/aptos-transaction-benchmarks/src/"

tps_set = {}
speedups_set = {}

fail = False
for threads in THREADS:
    operating_system = platform.system()
    if operating_system == "Linux":
        command = (
            f"taskset -c 0-{threads-1} cargo run --profile performance param-sweep"
        )
    else:
        command = f"cargo run --profile performance param-sweep"
    output = subprocess.check_output(
        command, shell=True, text=True, cwd=target_directory
    )
    print(output)

    for i, block_size in enumerate(BLOCK_SIZES):
        tps_index = i * 2
        speedup_index = i * 2 + 1
        key = f"{block_size}_{threads}"
        tps = int(re.findall(r"Avg Parallel TPS = (\d+)", output)[i])
        speedups = int(re.findall(r"Speed up (\d+)x over sequential", output)[i])
        tps_set[key] = tps
        speedups_set[key] = speedups
        tps_diff = (tps - THRESHOLDS[key]) / THRESHOLDS[key]
        if abs(tps_diff) > THRESHOLDS_NOISE:
            print(
                f"Parallel TPS {tps} {'below' if tps_diff < 0 else 'above'} the threshold {THRESHOLDS[key]} by {abs(tps_diff)*100:.0f}% (more than {THRESHOLDS_NOISE*100:.0f}%). Please "
                f"{'optimize' if tps_diff < 0 else 'increase the hard-coded TPS threshold since you improved'} the execution performance. :)\n"
            )
            fail = True

        for speedup_threshold, noise, above in (
            (SPEEDUPS[key], SPEEDUPS_NOISE_BELOW, False),
            (SPEEDUPS[key], SPEEDUPS_NOISE_ABOVE, True),
        ):
            if abs((diff := speedups - speedup_threshold) / speedup_threshold) > noise:
                print(
                    f"Parallel SPEEDUPS {speedups} {'below' if not above else 'above'} the threshold {speedup_threshold} by {noise} for {block_size} block size with {threads} threads!  Please {'optimize' if not above else 'increase the hard-coded speedup threshold since you improved'} the execution performance. :)\n"
                )
                fail = True

for block_size in BLOCK_SIZES:
    for threads in THREADS:
        key = f"{block_size}_{threads}"
        print(
            f"Average Parallel TPS with {threads} threads for {block_size} block: TPS {tps_set[key]}, Threshold TPS: {THRESHOLDS[key]}, Speedup: {speedups_set[key]}x, Speedup Threshold: {SPEEDUPS[key]}x"
        )

if fail:
    exit(1)
