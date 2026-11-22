#!/bin/sh
# Runs inside the boot job's containers: a one-file change, then the build through hushrun, timed.
set -eu
cd /home/runner/work/bake-demo/bake-demo
git config --global --add safe.directory "$PWD"
echo "== store at /hushrun:"; ls -ld /hushrun /hushrun/* 2>/dev/null || echo "none"
hushrun build status 2>&1 || true
# Crates are downloaded before the clock starts, so both containers time compiling alone.
cargo fetch --locked -q
# The one-file change a sandbox makes before it builds.
echo '// changed in the sandbox' >> src/main.rs
git diff --stat
start=$(date +%s.%N)
HUSHRUN_BUILD=on hushrun run build -- cargo build --release --locked
end=$(date +%s.%N)
echo "build wall seconds: $(echo "$end - $start" | awk '{print $1 - $3}')"
./target/release/bake-demo "a=1 b=2"
