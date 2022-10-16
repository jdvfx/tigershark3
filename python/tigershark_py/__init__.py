##!/usr/bin/env python3
import hou
import os
import json
from subprocess import Popen, PIPE
from datetime import datetime
from stat import S_IREAD, S_IRGRP, S_IROTH

# get tigershark executable
target="debug"
# target="release"

# for now the Rust executable path is hard-coded...
# will be added to a /tools/ dir
pwd = "/home/bunker/projects/tigershark3/target/"
ts_exe = pwd+target+"/tigershark3"

class TigerShark:

    def __init__(self,node):
        self.node = node

    # houdini
    def backup_hip(self):

        hipfile = hou.hipFile
        hipfile.save()

        ext = hipfile.path().split(".")[-1]
        datapath = self.node.evalParm("datapath")
        datapath = datapath.split("/")

        dataname = datapath.pop()
        dataname = dataname.split(".")[0]

        now = datetime.today().strftime("%Y%m%d_%H%M%S")
        dir = "/".join(datapath) + "/.tigershark/"
        file = dataname + "-"+now+"." + ext
        backupfile = dir + file 
        command = "mkdir -p "+ dir + " && cp " + hipfile.path() + " " + backupfile
        os.system(command)
        # set backup file to read only
        os.chmod(backupfile, S_IREAD|S_IRGRP|S_IROTH)

        return backupfile

    # houdini
    def increment_version(self):
        version = self.node.parm("version")
        version.set(version.eval()+1)

    # call rust executable, pass command and asset
    # return tuple with (ExitCode,Output)
    def ts(self,command,asset):
        try:
            process = Popen([ts_exe,"-c",command,"-a",json.dumps(asset)], stdout=PIPE)
            (output, err) = process.communicate()
            exit_code = process.wait()
            output = output.decode("utf-8")
            if exit_code == 0:
                return (0,output)
            else:
                return (1,output)
        except:
            return (1,"Python Popen failed")

    # grab all the houdini UI parms and create
    # an asset dictionnary, later dumpped as json
    def build_asset(self):
        asset_parms =  ["name","location","version","datapath"]
        asset = {}
        for parm in asset_parms:
            asset[parm] = self.node.evalParm(parm)
        return asset
    
    # tigershark -c source 
    def source(self):
        asset = self.build_asset()
        command = "source"
        return(self.ts(command,asset))

    # tigershark -c insert
    def insert(self):
        self.increment_version()

        source = self.backup_hip()
        asset = self.build_asset()
        asset["source"]=source
        command = "insert"

        output = self.ts(command,asset)
        if output[0] == 0:
            version = int(output[1])
            self.node.parm("version").set(version)
        return output


