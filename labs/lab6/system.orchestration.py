import sys
import paramiko
import time
import os.path as path

class Server():
    def __init__(self, config):
        self.host_ip = config.get('host_ip', None)
        self.username = config.get('username', None)
        self.port = config.get('port', 22)
        self.pkey = config.get('pkey', None)
        self.ssh_client = paramiko.SSHClient()
        self.transport = paramiko.Transport((self.host_ip, self.port))
        self.sftp_client = None

        self.ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    def ssh_connect(self):
        try:
            self.ssh_client.connect(self.host_ip, port=self.port, username=self.username, pkey=self.pkey)
            print(f"Successfully established SSH connection to {self.host_ip}")
        except Exception as e:
            print(f"Error creating SSH connection to {self.host_ip}: {e}")

    def sftp_connect(self):
        try:
            self.transport.connect(username=self.username, pkey=self.pkey)
            self.sftp_client = paramiko.SFTPClient.from_transport(self.transport)
            print(f"Successfully established SFTP connection to {self.host_ip}")
        except Exception as e:
            print(f"Error creating SFTP connection to {self.host_ip}: {e}")

    def close(self):
        try:
            self.ssh_client.close()
            self.sftp_client.close()
            self.transport.close()
            print(f"Successfully closed SSH and SFTP connections to {self.host_ip}")
        except Exception as e:
            print(f"Error closing connections: {e}")

    def get(self, remote_path, local_path):
        try:
            self.sftp_client.get(remote_path, local_path)
            print(f"Successfully retrieved {remote_path} from {self.host_ip}")
        except Exception as e:
            print(f"Error getting '{remote_path}' from {self.host_ip}: {e}")

    def put(self, local_path, remote_path):
        try:
            if path.exists(local_path):
                self.sftp_client.put(local_path, remote_path)
                print(f"Successfully sent {local_path} to {self.host_ip}")
            else:
                raise Exception(f"{local_path} not found.")
        except Exception as e:
            print(f"Error sending '{local_path}' to {self.host_ip}: {e}")

    def listdir(self, remote_path):
        try:
            return self.sftp_client.listdir(remote_path)
        except Exception as e:
            print(f"Cannot list files in '{remote_path}': {e}")
            return [];

    def exec_command(self, command=None):
        try:
            if command is None:
                raise Exception("No command specified")
            else:
                print(f"Successfully executed SSH command '{command}' on {self.host_ip}")
                return self.ssh_client.exec_command(command)
        except Exception as e:
            print(f"Error executing '{command}' on {self.host_ip}: {e}")



if __name__ == '__main__':

    host_ip = sys.argv[1]
    home_dir = '/home/student/'

    ssh_key_path = path.join(home_dir, '.ssh/id_rsa')
    input_path = path.join(home_dir, 'project1', 'input')
    output_path = path.join(home_dir, 'project1', 'output')
    tar_path = path.join(home_dir, 'project1.tar.gz')
    sh_path = path.join(home_dir, 'project1', 'project1.sh')

    pkey = paramiko.RSAKey.from_private_key_file(ssh_key_path)

    config = {
        'host_ip': host_ip,
        'username': 'student',
        'port': 22,
        'pkey': pkey
    }

    server = Server(config)

    server.ssh_connect()
    server.sftp_connect()

    print()

    server.put(tar_path, tar_path)
    server.exec_command(f'tar -xzf {tar_path} -C {home_dir}')
    server.exec_command(f'bash {sh_path} -i {input_path} -o {output_path}')

    print("\nWaiting for 65 seconds...")

    time.sleep(65)

    print("Successfully waited 65 seconds")
    print()

    metric_files = server.listdir(output_path)

    for file in metric_files:
        server.get(path.join(output_path, file), path.join(home_dir, file))