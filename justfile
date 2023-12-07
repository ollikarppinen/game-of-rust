run:
    cargo run --release

build:
    cargo build

bench:
    cargo bench

alloc:
    cargo instruments -t Allocations --time-limit 1000 --open

perf:
    cargo instruments -t Game Performance --time-limit 1000 --open

trace:
    cargo instruments -t 'System Trace' --time-limit 1000 --open

profile:
    cargo instruments -t 'Time Profiler' --time-limit 5000 --open --release
