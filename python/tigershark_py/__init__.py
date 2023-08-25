##!/usr/bin/env python3
import hou
import re
import os
import json
from subprocess import Popen, PIPE
from datetime import datetime
from stat import S_IREAD, S_IRGRP, S_IROTH

from tigershark_py.tsexec import get_ts_exec

# get tigershark executable
ts_exe = get_ts_exec(__file__)
if ts_exe is None:
    print("Tigershark executable not found")
    exit()


class TigerShark:

    def __init__(self,node):
        self.node = node

    # backup houdini .hip/.hipnc file
    def backup_hip(self):

        hipfile = hou.hipFile
        hipfile.save()

        hip_extension = hipfile.path().split(".")[-1]
        datapath = self.node.evalParm("datapath")
        datapath = datapath.split("/")

        dataname = datapath.pop()
        dataname = dataname.split(".")[0]

        now = datetime.today().strftime("%Y%m%d_%H%M%S")
        dir = "/".join(datapath) + "/.tigershark/"
        file = dataname + "-"+now+"." + hip_extension
        backupfile = dir + file
        # copy/rename current hipfile to backup directory (.tigershark) 
        command = "mkdir -p "+ dir + " && cp " + hipfile.path() + " " + backupfile
        os.system(command)
        # set backup file to read only
        os.chmod(backupfile, S_IREAD|S_IRGRP|S_IROTH)
        print(backupfile)
        return backupfile

    # call rust executable, pass command and asset
    # return tuple with (ExitCode,Output)
    def ts(self,command,asset):
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

    # grab all the houdini UI parms and create
    # an asset dictionnary, later dumpped as json
    def build_asset(self):
        asset_parms =  ["name","location","datapath","version"]
        asset = {}
        for parm in asset_parms:
            asset[parm] = self.node.evalParm(parm)
        print(">> " ,asset)
        return asset
   
    # Tigershark commands (CommandType enum)
    # Insert
    # Source
    # Delete
    # Latest
    # Approve

    # houdini - get all dependend node's version_id
    def get_depends(self):

        self_version_id = self.node.evalParm("version_id")
        depends = []
        d = hou.hscript("opdepend -iIe "+self.node.path())
        for i in d[0].split("\n"):
            n = hou.node(i)
            if n:
                if n.type().name()=="subnet":
                    try:
                        v = n.evalParm("version_id")
                        if v!= self_version_id:
                            depends.append(str(v))
                    except Exception:
                        pass
        return ",".join(depends)

    # tigershark -c insert
    def insert(self):

        asset = self.build_asset()
        # get latest version from DB and increment version on hda UI
        command = "latest"

        output = self.ts(command,asset)
        if output[0]==0:
            # all the cli outputs are string
            # eg: the second element is '"12"'
            v = re.sub('"','',output[1])
            if v.isdigit():
                self.node.parm("version").set(int(v)+1)

        else:
            # if version not found, then it's v0 
            self.node.parm("version").set(1)
            # return output

        # backup houdini file
        source = self.backup_hip()
        asset = self.build_asset()
        asset["source"]=source
        # get dependencies list, stash in asset
        depends = self.get_depends()
        asset["depend"]=depends

        command = "insert"
        output = self.ts(command,asset)
        return output

    # tigershark -c source 
    def source(self):
        asset = self.build_asset()
        print(f"\n{asset}\n")
        command = "source"
        return(self.ts(command,asset))

    # tigershark -c delete
    def delete(self):
        asset = self.build_asset()
        command = "delete"
        return(self.ts(command,asset))

    # tigershark -c latest
    def latest(self):
        asset = self.build_asset()
        command = "latest"
        return(self.ts(command,asset))

    # tigershark -c approve
    def approve(self):
        asset = self.build_asset()
        command = "approve"
        print(asset)
        return(self.ts(command,asset))

    # tigershark -c source (and open hip file)
    def open_source(self):
        asset = self.build_asset()
        command = "source"

        output = self.ts(command,asset)
        if output[0] == 0:
            source_file = output[1]
            # need to test if this works and asks to save
            # the current hip file before openning the backup
            hou.hipFile.load(source_file)


