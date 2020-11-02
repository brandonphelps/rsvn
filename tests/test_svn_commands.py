from rsvn.svn_commands import log, set_username_password
from .generate_expected_responses import get_storage_by_cmd, load_output
import os

SERVER_PATH = 'svn://localhost:3690/Test'
USERNAME  = 'user'
PASSWORD = 'user'

expected_responses_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "svn_expected_responses")


def expected_response(params):
    with open(os.path.join(expected_responses_dir, 'log_1', 'data.txt'), 'r') as reader:
        return reader.read()

set_username_password(USERNAME, USERNAME)

def test_svn_log():
    assert log(SERVER_PATH, 0) == load_output(get_storage_by_cmd('logs', ['svn', 'log', f"{SERVER_PATH}@{0}"]))
    assert log(SERVER_PATH, 1) == load_output(get_storage_by_cmd('logs', ['svn', 'log', f"{SERVER_PATH}@{1}"]))
    assert log(SERVER_PATH, 2) == load_output(get_storage_by_cmd('logs', ['svn', 'log', f"{SERVER_PATH}@{2}"]))

