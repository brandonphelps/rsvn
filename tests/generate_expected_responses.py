import hashlib
import os
import subprocess

SERVER = 'svn://localhost:3690/Test'

# todo: chane to user input?
STORAGE_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "svn_expected_responses")

def svn_cmds_to_hash(cmd):
    f_cmd = ''.join(cmd)
    s = hashlib.sha256()
    s.update(f_cmd.encode('utf-8'))
    return s.hexdigest()

def get_storage_by_cmd(cmd_base, cmd):
    hash_p = svn_cmds_to_hash(cmd)
    pre = hash_p[:2]
    post = hash_p[2:]
    print(f"Pre: {pre}")
    print(f"Post: {post}")
    return os.path.join(STORAGE_DIR, cmd_base, os.path.join(pre, post))

def save_output(cmd_output, path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w') as writer:
        writer.write(cmd_output)

def load_output(path):
    with open(path, 'r') as reader:
        return reader.read()

def generate_logs():
    for i in range(3):
        cmd = ['svn', 'log', f"{SERVER}@{i}"]
        outpu = subprocess.check_output(cmd).decode('utf-8')
        storage_place = get_storage_by_cmd('logs', cmd)
        save_output(outpu, storage_place)
        print(f"Saving to: {storage_place}")


if __name__ == "__main__":
    generate_logs()
