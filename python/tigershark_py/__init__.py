##!/usr/bin/env python3
import json
from subprocess import Popen, PIPE

# get tigershark executable
target="debug"
# target="release"

# for now the Rust executable path is hard-coded...
pwd = "/home/bunker/projects/tigershark3/target/"
ts_exe = pwd+target+"/tigershark3"

# return tuple with (ErrorCode,output)
def tigershark(command,asset):
    try:
        process = Popen([ts_exe,"-c",command,"-a",json.dumps(asset)], stdout=PIPE)
        (output, err) = process.communicate()
        exit_code = process.wait()
        output = output.decode('utf-8')
        if exit_code == 0:
            return (0,output)
        else:
            return (1,output)
    except:
        return (1,"Python Popen failed")

