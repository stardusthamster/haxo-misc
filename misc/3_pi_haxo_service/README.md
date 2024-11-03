# Pi Haxo Service for Switching Instruments
In this project we create some scripts and different haxo.service config files. When we restart the haxo service with the file for flute for example, then the haxophone uses the flute sound and plays in C.


## Preparation:
Connect the haxophone to your computer via ethernet.

Copy the following files from this directory to the haxophone:

```
scp haxo.service.* pi@haxophone.local:/home/pi
scp haxo_restart.sh pi@haxophone.local:/home/pi
```

Login to haxophone:

```
ssh pi@haxophone
```

On the haxophone make the restart script executable:

```
chmod u+x haxo_restart.sh
```

Update the /usr/local/bin/create_midi_gadget.sh file (the old code in there stopped the haxo service from restarting. The new one contains "usb midi gadget already exists" ):

```
wget https://raw.githubusercontent.com/cardonabits/haxo-rs/refs/heads/main/scripts/systemd/create_midi_gadget.sh

sudo mv create_midi_gadget.sh /usr/local/bin/
sudo chmod a+x /usr/local/bin/create_midi_gadget.sh
```
## Description
Short description of each file in this directory

- haxo_restart.sh -> the script for restarting with a new configuration
- haxo.service.56 -> Trumpet
- haxo.service.65 -> Alto Sax
- haxo.service.71 -> Clarinet
- haxo.service.72 -> Piccolo Flute
- haxo.service.73 -> Flute
- haxo.service.orig -> the original using tenor sax

## Usage

Switch haxophone to flute:

```
./haxo_restart.sh 73
```

Switch back to original tenor sax:

```
./haxo_restart.sh orig
```



If there are problems, look into the log:

```
journalctl -u haxo -f
``` 
or for reverse

```
journalctl -u haxo -r
```

and have a look at the service status

```
systemctl status haxo.service
```

## Appendix

### How the transpose value is defined
From the transpose source code of the haxophone software:

To set the transpose directly from a note we measure how far away from
the reference the note is. This is then used as the offset. We use high C
as the reference so we can fit soprano (High Bb -> -2), alto (Mid Eb -> -9),
tenor (Mid Bb, -> -14) and bari (Low Eb -> -21) within the reachable range.

With this information we know what transpose number to use in the haxo.service config file:

- transpose 0 for flute, as high C is the reference
- transpose 12 for piccolo flute
- transpose -2 for clarinet as it is a high Bb instrument


### How the prog-number for the instrument is defined
For the prog-number to use for each instrument see the list of haxophone sounds in the haxophone manual:
 [https://github.com/cardonabits/haxo-hw/blob/main/docs/manual/manual.md#haxophone-sounds](https://github.com/cardonabits/haxo-hw/blob/main/docs/manual/manual.md#haxophone-sounds)



