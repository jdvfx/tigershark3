##!/usr/bin/env python3
# import os
# import json
from subprocess import Popen, PIPE
import re

# get tigershark executable
target="debug"
# target="release"

# for now the Rust executable path is hard-coded...
# will be added to a /tools/ dir
pwd = "/home/bunker/projects/tigershark3/target/"
ts_exe = pwd+target+"/tigershark3"

class TigerShark:

    def __init__(self):
        pass
    # call rust executable, pass command and asset
    # return tuple with (ExitCode,Output)
    def ts(self,command):
        try:
            process = Popen([ts_exe,"-c",command], stdout=PIPE)
            (output, _err) = process.communicate()
            exit_code = process.wait()
            output = output.decode("utf-8")
            if exit_code == 0:
                return (0,output)
            else:
                return (1,output)
        except:
            return (1,"Python Popen failed")

ts = TigerShark()
purge_list= ts.ts("purge")
print()
if purge_list[0] == 0:
    files_to_purge = purge_list[1][1:-1].split("#")
    for purge_file in files_to_purge:
        print(purge_file)
else:
    print("Error in purge list")


