#! /bin/sh
sudo systemctl stop haxo
sudo cp haxo.service.$1 /etc/systemd/system/haxo.service 
sudo systemctl daemon-reload
sudo systemctl start haxo

