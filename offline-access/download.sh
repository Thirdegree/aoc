#!/opt/homebrew/bin/bash
url="https://adventofcode.com"
year=2019
url_day="${url}/${year}/day"
which curl

mkdir aoc
cd aoc

mkdir static
curl "${url}/static/style.css" > static/style.css

mkdir -p ${year}/day
cd ${year}/day

for i in $(seq 16 25)
do
        mkdir $i
        echo day $i
        curl "${url_day}/${i}" > ${i}/index.html
        curl --cookie "session=53616c7465645f5f3d56bf9983ff062a3a0c4018145a6bc2f3391c5d84a6dbea95b7b8cb9baa014b5ae4f22b4b2a4bcff2bba8331b9cbdee63267c4ef9ca3aba" "${url_day}/${i}/input" > ${i}/input
        sleep 10
done

