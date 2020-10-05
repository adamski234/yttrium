#!/bin/sh
for dir in std_*
do
	cargo build -p $dir $@
	# Check for `release` argument to determine the directory to take the files from
	if [[ "$@" == *"--release"* ]]
	then
		cp ./target/release/lib$dir.so ./keys
	else
		cp ./target/debug/lib$dir.so ./keys
	fi
done