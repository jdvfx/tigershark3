##!/usr/bin/env python3
import hou
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


class TigerShark:

    def __init__(self,node):
        self.node = node

    def ts(self,command,asset):
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

    def source(self):

        name = self.node.evalParm("name")
        location = self.node.evalParm("location")
        version = self.node.evalParm("version")
        #
        asset = {
            "name": name,
            "location": location,
            "version": version,
        }
        command = "source"

        return(self.ts(command,asset))

    def insert(self):

        name = self.node.evalParm("name")
        location = self.node.evalParm("location")
        datapath = self.node.evalParm("datapath")
        source = hou.hipFile.path()
        #
        asset = {
            "name": name,
            "location": location,
            "datapath": datapath,
            "source": source,
        }
        command = "insert"

        output = self.ts(command,asset)
        if output[0] == 0:
            version = int(output[1])
            self.node.parm("version").set(version)
        return output


