#!/usr/bin/bash

rustc a.rs

input1="
"
ans1=""

input2="
"
ans2=""

input3="
"
ans3=""

input4="
"
ans4=""

input5="
"
ans5=""

try() {
	echo "actual:"
	./a << _EOT_
$1
_EOT_

	echo "expected:"
	echo "$2"
	echo "----------"
}

try "$input1" "$ans1"
try "$input2" "$ans2"
try "$input3" "$ans3"
try "$input4" "$ans4"
try "$input5" "$ans5"
