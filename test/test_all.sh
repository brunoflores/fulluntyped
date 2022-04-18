#!/usr/bin/env bash

RED='\033[0;31m'
GREEN='\033[0;32m'
NOCOLOR='\033[0m'

for test in ./test/*.f; do
    fname_out=$(basename $test ".f").out
    fname_exp=$(basename $test ".f").exp

    dune exec ./bin/main.exe -- $test >./test/$fname_out

    # To promote, uncomment:
    # cp ./test/$fname_out ./test/$fname_exp

    DIFF=$(diff ./test/$fname_out ./test/$fname_exp)
    if [ "$DIFF" != "" ]; then
        echo ""
        echo "-------------------------"
        printf "${RED}FAILED DIFF:${NOCOLOR} $test\n"
        echo $DIFF
        echo "-------------------------"
        exit 0
    else
        printf "${GREEN}PASSED:${NOCOLOR} $test\n"
    fi
done

exit 0
