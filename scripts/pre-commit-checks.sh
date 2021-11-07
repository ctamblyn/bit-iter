#!/bin/bash

run_test() {
    local dots
    local tmpfile
    local cmd="cargo --color always $@"

    dots=$(head -c 74 < /dev/zero | tr '\0' '.')
    tmpfile=$(mktemp pre-commit-actions.sh.XXXXXX)

    echo -n "Running \`$cmd\` $dots" | head -c 74
    echo -n " "

    script -e -q -c "cargo clean && $cmd" "$tmpfile" >/dev/null

    if [ "$?" -eq 0 ] ; then
        echo -e "\e[32mPASS\e[0m"
    else
        echo -e "\e[31mFAIL\e[0m\n\nOutput of \`$cmd\`:\n"
        cat "$tmpfile"
        echo
    fi

    rm "$tmpfile"
}

run_test check
run_test test
run_test fmt --all -- --check
run_test clippy -- -D warnings
