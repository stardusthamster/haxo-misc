# haxo-misc
This repository contains modifications and addons for the haxophone by @cardonabits (see [https://github.com/cardonabits/haxo-rs](https://github.com/cardonabits/haxo-rs))

## Modifications

### haxo.service
This file replaces /etc/systemd/system/haxo.service

I modified the haxo.service to use other startup parameters and replaced the original file with the file provided in this repository.

The haxophone now uses the flute sound and transposes everything by an octave.


### notemap
This file replaces /usr/share/haxo/notemap.json

The notemap.json.modified uses the original notemap.json file in concert pitch and adds more fingerings (for this task I used the notemap viewer [https://haxo-notemap.nn.r.appspot.com](https://haxo-notemap.nn.r.appspot.com), for more informations see discussion here [https://github.com/cardonabits/haxo-rs/discussions/18](https://github.com/cardonabits/haxo-rs/discussions/18). 
The changes to the notemap.json add some fingerings flute players are used to as to make it easier for them to play the haxphone.

