#!/usr/bin/env bash

# ------ Functions ------

get_input() {
    local label="$1"
    read -p "$label: " value
    echo "$value"
}

get_info() {
    local label="$1"
    local value=$(get_input "$label")

    while [[ -z "$value" ]]; do
        value=$(get_input "$label")
    done

    echo "$value"
}

letter_writer() {
    local file_path="$1"
    local full_name="$2"
    local department="$3"
    local job_title="$4"

cat << EOT > "$file_path"
Dear ${full_name%% *},

Welcome to Initech Corporation! We're so happy to have you in the $department Department as a $job_title.
Please don't forget to complete your TPS Report in a timely manner.

Sincerly,

Bill Lumbergh
EOT
}

file_system_writer() {
    local user_dir="$1"

    mkdir -p $user_dir/Desktop $user_dir/Documents $user_dir/Pictures $user_dir/Downloads

    if [ -f $HOME/swingline.jpg ]; then
        cp $HOME/swingline.jpg $user_dir/Pictures/
    else
        echo "file swingline.jpg not found in $HOME"
    fi
}

permission_editor() {
    local $user_dir="$1"
    local $owner="$2"
    local $group="$3"

    if [ -f $HOME/swingline.jpg ]; then
        chmod 444 $user_dir/Documents/welcome.txt
    fi

    chown -R $owner:$group "$user_dir"
}

create_user() {
    local username="$1"
    local full_name="$2"
    local department="$3"
    local job_title="$4"

    local user_dir="/home/$username"

    useradd -c "$full_name" -d "$user_dir" $username

    file_system_writer "$user_dir"

    letter_writer "$user_dir/Documents/welcome.txt" "$full_name" "$department" "$job_title"

    permission_editor $user_dir "test_user" "test_user" 2> /dev/null
}

# ------ Start of Script ------

## ------ Check sudo ------

if [[ $EUID -ne 0 ]]; then
    echo "You must run this script as a super user (sudo)"
    exit 1
fi

## ------ Get Information From Input and Create User ------

while true; do
    username=$(get_info "Username")
    full_name=$(get_info "Full Name")
    department=$(get_info "Department")
    job_title=$(get_info "Job Title")

    create_user "$username" "$full_name" "$department" "$job_title"

    echo -e "\nUser $username added!"

    is_add_another=$(get_input "Would you like to add another user? (y/n)")

    while [[ "$is_add_another" != "y" && "$is_add_another" != "n" ]]; do
        echo "Please enter either 'y' or 'n'"
        is_add_another=$(get_input "Would you like to add another user? (y/n)")
    done

    [[ "$is_add_another" == "n" ]] && break
done

# ------ End of Script ------

exit 0