#!/usr/bin/env bash

function compat_weapons() {
    bat raw/upgrades.json -p \
        | grep compatName \
        | sort \
        | uniq \
        | tr -s ' ' \
        | cut -d' ' -f3- \
        | sed -E 's/"\s*([^"]+)",/\1/g'
}

function json_structure() {
    if [[ ! "$1" =~ ^[^.]+.json$ ]]; then
        echo "Not a JSON file: '$1'"
        return 1
    fi

    jq -r 'paths | map(tostring) | join(".")' "$1" \
        | cut -d'.' -f3- \
        | sort \
        | uniq
}

commands=("compat_weapons" "json_structure")
arg="\<$1\>"

if [[ ${commands[@]} =~ $arg ]]; then
    command="$1"
    shift 1
    $command $@
else
    echo "Invalid command: '$1'. Valid commands are: ${commands[@]}"
fi

