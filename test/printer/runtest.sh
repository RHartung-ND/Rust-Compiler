#!/bin/bash
show_correct="True"

if [ $1 ] && ([ ${1,,} == "false" ] || [ ${1,,} == "f" ]); then
    show_correct="False"
fi

for testfile in test/printer/good*.bminor; do
	if target/release/bminor --parse $testfile > $testfile.out; then
		if [ $show_correct == "True" ]; then
            echo "$testfile success (as expected)"
        fi
    else
        echo "$testfile failure (INCORRECT)"
	fi
done
