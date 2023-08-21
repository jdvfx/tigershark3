from typing import Optional,List
import os 

# find latest Tigershark executable
# from the Rust release or debug directories

def get_ts_exec(cwd) -> Optional[str]:

    cwd_split:List[str] = cwd.split("python")
    if cwd_split==1:
        return None
    ts_path:str = cwd_split[0]

    rust_excutables = ["debug","release"]

    ts_execs = []
    for i in rust_excutables:
        ts_exec = os.path.join(ts_path,f"target/{i}/tigershark3")
        if os.path.isfile(ts_exec):
            mtime = os.path.getmtime(ts_exec)
            ts_execs.append([mtime,ts_exec])
    ts_execs.sort(reverse=True)

    if len(ts_execs)>0:
        return ts_execs[0][1]
    else:
        return None

