#!/usr/bin/env bash

# Functions

check_deps() {
    local package="$1"
    if ! command -v $package &> /dev/null; then
        echo "$package is not installed. Installing ansible..."
        sudo dnf install -y $package
        echo "$package successfully installed!"
    else
        echo "$package is already installed. Skipping..."
    fi
    echo
}

create_path() {
    local path="$1"
    local is_sudo="$2"

    if [[ -d "$path" ]]; then
        echo "$path already exists. Skipping..."
        return 0
    fi

    if [[ "$is_sudo" == true ]]; then
        sudo mkdir -p "$path"
    else
        mkdir -p "$path"
    fi

    echo "$path successfully created!"
}

copy() {
    local src="$1"
    local dest="$2"

    if [[ ! -e "$src" ]]; then
        echo "$src does not exist."
        return 1
    fi

    sudo cp -r "$src" "$dest"

    if [[ $? -ne 0 ]]; then
        echo "Copy failed"
        return 1
    fi

    echo "$src successfully copied in $dest"
}

# Variables

client_dist="./client/dist"
nginx_config="./config/nginx.conf"
server_bin="./server/target/release/server"
ansible_playbook="./playbook.yaml"

client_remote="/var/www/project2/public"
rans_remote="/etc/rans"
bin_remote="/usr/bin"

# Start of script

echo "============ Checking Dependencies ============"
echo
check_deps "epel-release"
check_deps "ansible"

echo
echo "============ Creating Remote Paths ============"
echo

create_path "$rans_remote" true
create_path "$client_remote" true

echo
echo "============ Run Ansible Playbooks ============"
echo

ansible-playbook -i "$ansible_playbook"

echo
echo "============ Setup Files ============"
echo

copy "$nginx_config" "$rans_remote"
echo

if [[ ! -e "$client_dist" ]];
    echo "Building client app..."
    (cd client && npm run build)
    echo "Client app successfully built!"
    echo
fi

copy "$client_dist/*" "$client_remote"
echo

if [[ ! -e "$server_bin" ]];
    echo "Building server binary..."
    (cd server && cargo build --release --bin server)
    echo "Server binary successfully built!"
    echo
fi

copy "$server_bin" "$bin_remote"
echo

echo "Successfully executed script!"
echo

# End of Script