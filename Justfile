set dotenv-load := true

run day='':
    RUST_LOG=debug cargo run --bin "day$(just _day {{ day }})"

bench day='':
    cargo run --release --bin "day$(just _day {{ day }})"

test day='':
    RUST_LOG=debug cargo test --bin "day$(just _day {{ day }})"

expensive-tests day='':
    RUST_BACKTRACE=1 RUST_MIN_STACK=8388608 cargo test --bin "day$(just _day {{ day }})" -- --ignored

lint:
    pre-commit run -a

watch day='':
    cargo watch -s "cargo test --bin \"day$(just _day {{ day }})\""

prepare day='':
    #! /bin/sh

    set -eu

    day="$(just _day {{ day }})"
    just get "{{ day }}"
    git co -b "day${day}"

get day='':
    #! /bin/sh

    set -eu

    day="$(just _day {{ day }})"
    filename="day${day}"

    curl --fail --cookie "session=${SESSION_COOKIE:?Session cookie unavailable}" "https://adventofcode.com/2025/day/$((10#${day}))/input" > "input/${filename}.txt"

@_day day='':
    if [ -n "{{ day }}" ]; then \
        printf "%02d" "{{ day }}"; \
    else \
        date "+%d"; \
    fi
