sudo /etc/rc.d/init.d/sesinetd start
cd /opt/hfs19.5
bash houdini_setup_bash
export PYTHONPATH=$PYTHONPATH:/home/bunker/projects/tigershark3/python
bin/houdinifx


