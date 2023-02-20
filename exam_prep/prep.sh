#!/usr/bin/env bash

ip_addresses() {
    ips=$(cat apache_log.txt | cut -d ' ' -f 1 | sort -n | uniq)

    echo "$ips"
    echo

    count="$(echo "$ips" | wc -l)"
    echo $count
    echo

    echo "$ips" | grep -E "^129\.21\."
    echo

    grep -E "^129\.21\." < apache_log.txt | cut -d ' ' -f 1 | sort -n
    echo
}

search_for() {
    local search_term="$1"
    local count=$(grep -i "$search_term" apache_log.txt | wc -l)
    local formatted="'$search_term' found $count times"

    echo "$formatted"

    if [ ! -f log.txt ]; then
        echo "$formatted" > log.txt
    else
        echo "$formatted" >> log.txt
    fi
}

if [ $# -lt 1 ]; then
    search_for "doodle"
else
    for param in $@; do
        search_for $param
    done
fi



#ip_addresses

