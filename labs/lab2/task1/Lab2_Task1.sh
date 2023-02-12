#!/usr/bin/env bash

# How to use it
# ./Lab2_Task1 <num_rands> [<min> <max>]

# ------ Functions ------

num_writer() {
	local file="$1"
	local num="$2"

	if [ -z "$file" ]; then
		echo "Missing parameter (1): File name"
		return 1
	fi

	if [ -z "$num" ]; then
		echo "Missing  paramter (2): Number"
		return 1
	fi

	echo "$num" >> "$file"
}

get_random_num() {
    local min="$1"
    local max="$2"

    # Calculate and return a random number between min and max (inclusive)
    echo $((min + $RANDOM % ((max - min) + 1)))
}

# ------ Start of Script ------

## ------ Handle parameters ------

num_rands=$1
min=$2
max=$3

echo -e "\n===== Parameters Handling =====\n"

# Validate num_rands
if [ -z "$num_rands" ]; then
	echo "Missing required argument (1)"
	exit 1
fi

if ! [[ $num_rands =~ ^[0-9]+$ ]]; then
    echo "Required argument (1) is not a positive integer."
    exit 1
fi

if [ -n "$min" ] && [ -n "$max" ]; then
    echo "You requested $num_rands numbers between $min and $max"
else
    echo "You requested $num_rands numbers."
fi

# Validate min
if [ -z "$min" ]; then
	min=1
else
    if ! [[ $min =~ ^[0-9]+$ ]]; then
        echo "Min (2) is not a positive integer."
        exit 1
    fi
fi

# Validate max
if [ -z "$max" ]; then
	max=32766
else
    if ! [[ $max =~ ^[0-9]+$ ]]; then
        echo "Max (3) is not a positive integer."
        exit 1
    fi
fi

if [ $min -ge $max ]; then
    echo "Min (2) cannot be greater than max  (2) ."
    exit 1
fi

## ------ Create File ------

output_file="rands_$num_rands.txt"

# if the file exists, it needs to clear it
echo -n > $output_file

## ------ Generate Numbers ------

echo -e "\n===== Number Generation =====\n"

sum=0
fin_min=$max
fin_max=$min

for i in $(seq 1 $num_rands); do
    rand_num=$(get_random_num $min $max)
    num_writer $output_file $rand_num

    sum=$((sum + rand_num))

    [[ $rand_num -lt $fin_min ]] && fin_min=$rand_num
    [[ $rand_num -gt $fin_max ]] && fin_max=$rand_num
done

average=$(awk "BEGIN {printf \"%.2f\", $sum / $num_rands}")
## ------ Statistics ------

echo "The smallest value generated was $fin_min"
echo "The largest value generated was $fin_max"
echo -e "The average value generated was $average\n"

## ------ End of Script ------

exit 0
