#!/bin/bash

run_test() {
    local width=78
    local dots
    local tmp

    dots=$(head -c "$width" < /dev/zero | tr '\0' '.')
    tmp=$(mktemp pre-commit-actions.sh.XXXXXX)

    echo -n "$* $dots" | head -c "$width"
    echo -n " "

    if script -eqc "cargo clean && cargo --color always $*" "$tmp" >/dev/null
    then
        echo "🎉"
    else
        echo "😭"
        echo -e "\nOutput of last command:\n"
        cat "$tmp"
        echo
    fi

    rm "$tmp"
}

run_test rustc --manifest-path=check_no_std/Cargo.toml -- -C link-arg=-nostartfiles
run_test check
run_test test
run_test fmt --all -- --check
run_test clippy -- -D warnings
