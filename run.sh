#!/bin/bash

runstr="cargo run"
input="input"

usage() { echo "Usage: [-tpr] [-d <int>]" 1>&2; exit 1; }

while getopts "ptrd:" opt ; do
    case $opt in
        p) 
            runstr="time ${runstr}"
            ;;
        t) 
            input="test"
            ;;
        r)
            runstr="${runstr} --release"
            ;;
        d)
            day=$OPTARG
            ;;
        *)
            usage
            ;;
    esac
done

runstr="${runstr} --bin day${day} src/days/day${day}/resources/${input}"

echo $runstr
$runstr
