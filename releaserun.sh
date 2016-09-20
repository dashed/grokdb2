#!/usr/bin/env bash
#
cargo build --release
# RUST_BACKTRACE=1 target/debug/gtdtxt reference-todo.gtd
# target/debug/gtdtxt empty.gtd
# RUST_BACKTRACE=1 target/debug/gtdtxt test2.gtd
# RUST_BACKTRACE=1 target/debug/gtdtxt test2.gtd -x -k=bar
# -x --due-within=2day --show-flagged --show-with-project=foo
# --show-overdue --show-incomplete --show-deferred --show-done


RUST_BACKTRACE=1 target/release/grokdb2 -p 3000 test.db
# RUST_BACKTRACE=1 target/debug/gtdtxt test3.gtd
