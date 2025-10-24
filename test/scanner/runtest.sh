#!/bin/bash
show_correct="True"

if [ $1 ] && ([ ${1,,} == "false" ] || [ ${1,,} == "f" ]); then
    show_correct="False"
fi

for testfile in test/scanner/good*.bminor; do
	if target/release/bminor --scan $testfile > $testfile.out; then
		if [ $show_correct == "True" ]; then
            echo "$testfile success (as expected)"
        fi
    else
        echo "$testfile failure (INCORRECT)"
	fi
done

for testfile in test/scanner/bad*.bminor; do
    if target/release/bminor --scan "$testfile" > "$testfile.out"; then
        echo "$testfile success (INCORRECT)"
    else
        if [ $show_correct == "True" ]; then
            echo "$testfile failure (as expected)"
        fi
    fi
done