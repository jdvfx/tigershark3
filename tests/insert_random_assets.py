import time
import string
import random
import json
from pathlib import Path
from subprocess import Popen, PIPE

parentdir = Path().resolve().parent
ts_exe = f"{parentdir}/target/debug/tigershark3"

def rand_name() -> str:
    fxnames = ["door", "window", "rock","car","dog","explosion","smoke","light","vol","large_splash",
     "splash","spray","mist","debris","source","pyro","gas","echo","sphere",
     "waterfall","plane","element","particles","ground","proxy","mesh","volume",
     "geo","boundingbox","mask","dry","random","color","full","lowpoly","fire","meshlight"]
    r= ''.join(random.SystemRandom().choice(string.ascii_letters + string.digits) for _ in range(3))
    n1 = random.choice(fxnames) 
    n2 = random.choice(fxnames) 
    return f"{n1}_{n2}_{r}"

def build_asset() -> dict:
    n = rand_name()

    asset = {
            "name":f"{n}",
            "location":f"{n}_loc",
            "datapath":f"datapath_{n}",
            "source":f"source_{n}",
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


command = "insert"

# how many assets to create
for i in range(10):

    rf = random.random()
    if rf>0.8:
        # create multiple versions of that asset
        ri = random.randint(1,10)
    else:
        ri = 1
    asset = build_asset()

    datapath = asset["datapath"]
    for j in range(ri):

        exts = ['bgeo','bgeo.sc','vdb','rat','pic','exr']
        ext = random.choice(exts)

        asset["datapath"] = f"{datapath}_v{j+1}.####.{ext}"
        clioutput = ts(command,asset)
        print(command,asset)
        print(clioutput)
        sleep = random.random()
        time.sleep(sleep)

