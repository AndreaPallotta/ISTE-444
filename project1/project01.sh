#!/usr/bin/env bash

# ------------ Variables ------------

input_folder="."
output_folder="."
executables=()
pids=()

# ------------ Functions ------------

install_deps() {
    if ! command -v ifstat &>/dev/null; then
        echo "ifstat not found. Installing it..."
        sudo yum install -y ifstat
        echo "ifstat installed correctly."
    fi

    if ! command -v iostat &>/dev/null; then
        echo "iostat not found. Installing it..."
        sudo yum install -y iostat
        echo "iostat installed correctly."
    fi
}

show_help_message() {
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo "APM Tool to monitor process and system level metrics"
    echo ""
    echo "Options:"
    echo "  -h, --help                Display this help message"
    echo "  -i, --input <folder>      Specify the folder where the c programs are placed (default: ./)"
    echo "  -o, --output <folder>     Specify the folder to output files to (default: ./)"
    echo ""
    echo "Examples:"
    echo "  $0 -i c_folder -o csv_folder"
    echo "  $0 -o /var/logs/project"
}

compile_c_scripts() {
    for file in "$input_folder"/*.c; do
        local output_file_name="${file%.*}"
        gcc "$file" -o "$output_file_name"
        executables+=( "$output_file_name" )
    done
}

parse_flags() {
    for arg in "$@"; do
        case $arg in
            -h|--help)
                show_help_message
                exit 0
                ;;
            -o|--output)
                output_folder="$arg"
                if [ ! -d "$output_folder" ]; then
                    mkdir -p "$output_folder"
                fi
                ;;
            -i|--input)
                input_folder="$arg"
                if [ ! -d "$input_folder" ]; then
                    echo "Input folder not found"
                    exit 1
                fi
                ;;
            *)
                echo "Unknown flag: $arg"
                ;;
        esac
    done
}

start_processes() {
    if [ ${#executables[@]} -eq 0 ]; then
        echo "Error: No C executables provided."
        exit 1
    fi

    for executable in "$executables"; do
        ./"$executable" &
        pids+=( $! )
    done
}

append_to_file() {
    local file_name="$1"
    local content="$2"

    echo "$content" >> "$output_folder/$file_name"
}

write_ps_info_to_csv() {
    local pid="$1"
    local file_name="$2"

    local cpu=$(ps -p "$pid" -o %cpu=)
    local mem=$(ps -p "$pid" -o %mem=)

    append_to_file "$file_name" "$(date +%s),$cpu,$mem"
}

get_process_metrics() {
    for pid in "${pids[@]}"; do
        proc_name=$(ps -p "$pid" -o comm=)

        file_name="${proc_name}_metrics.csv"
        echo "seconds,%CPU,%memory" > "$output_folder/$file_name"

        while true; do
            cpu=$(ps -p "$pid" -o %cpu=)
            mem=$(ps -p "$pid" -o %mem=)

            append_to_file "$file_name" "$(date +%s),$cpu,$mem"
            sleep 5
        done
    done
}

get_system_metrics() {
    local file_name="system_metrics.csv"

    echo "seconds,RX data rate,TX data rate,disk writes,available disk capacity" > "$output_folder/$file_name"

    while true; do
        rx=$(ifstat -q 1 1 | awk 'NR==3{print $1}')
        tx=$(ifstat -q 1 1 | awk 'NR==3{print $2}')
        disk_writes=$(iostat -d -k 1 2 | awk 'NR==2{print $4}')
        disk_capacity=$(df -m / | awk 'NR==2{print $4}')

        append_to_file "$file_name" "$(date +%s),$rx,$tx,$disk_writes,$disk_capacity"
        sleep 5
    done
}

cleanup() {
    for pid in "${pids[@]}"; do
        kill "$pid"
    done
}

# ------------ Start of Script ------------

trap cleanup INT TERM

parse_flags "$@"

install_deps
compile_c_scripts

start_processes

get_process_metrics &
get_system_metrics &

wait

# ------------ End of Script ------------
