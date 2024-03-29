#!/bin/sh

day="$1"

# If the "day" variable is not set, set it to the current day of the month
if [[ -z "$day" ]]; then
  day=$(date +%-d)
  echo "Info: No argument provided, using current day of the month: $day"
fi

# Check if the "day" variable is a number in the range 1-25
re='^(1?[1-9]|1[0-9]|2[0-5])$'
if ! [[ $day =~ $re ]]; then
  echo "Error: Argument must be a number in the range 1-25"
  exit 1
fi

# Check if the src/day{n}.rs file already exists
if [[ -f "src/day$day.rs" ]]; then
  echo "Error: src/day$day.rs already exists"
  exit 1
fi

# Copy day.template to src/day{n}.rs
cp day.template "src/day$day.rs"

# Create input.txt and test.txt files in inputs/day{n}
mkdir -p "inputs/day$day"
touch "inputs/day$day/test.txt" "inputs/day$day/input.txt"


prev_day="$(($day-1))"

# Add new module to src/main.rs file and define solvers
sed -i '' -E "s/mod day$prev_day;/mod day$prev_day;\nmod day$day;/g" "src/main.rs"
sed -i '' -E "s/define_solvers\!\((.*)\);/define_solvers!(\1, day$day);/g" "src/main.rs"