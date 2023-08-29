import os
import random
import json
from typing import Optional
from subprocess import Popen, PIPE

thisdir = os.path.dirname(__file__)
ts_exe = f"{thisdir}/../target/debug/tigershark3"

def get_random_asset() -> Optional[str]:
    db = os.environ['TS_DATABASE_URL']
    command = f"sqlite3 {db}"
    c = command.split(" ")
    x = 'SELECT name,location FROM assets'
    c.append(x)
    lines = []
    try:
        process = Popen(c, stdout=PIPE)
        (output, _err) = process.communicate()
        _exit_code = process.wait()
        output = output.decode("utf-8")
        lines = output.splitlines()
    except:
        return None

    if len(lines)>0:
        return random.choice(lines) 
    return None


def build_asset(asset_string:str) -> dict:
    r = asset_string.split("|")
    asset = {
            "name":f"{r[0]}",
            "location":f"{r[1]}",
            }
    return asset

def ts(command,asset):
    try:
        process = Popen([ts_exe,"-c",command,"-a",json.dumps(asset)], stdout=PIPE)
        (output, _err) = process.communicate()
        exit_code = process.wait()
        output = output.decode("utf-8")
        if exit_code == 0:
            return (0,output)
        else:
            return (1,output)
    except:
        return (1,"Python Popen failed")
#
#

command = "latest"
random_asset:Optional[str]= get_random_asset()
print(random_asset)
if random_asset != None:
    asset = build_asset(random_asset)
    clioutput = ts(command,asset)
    print(command,asset)
    print(clioutput)

