# Misc
The misc folder contains different programs used for a step by step approach.

The goal is to connect an arduino to the raspberry pi zero with the haxophone hat and then to push some button on the arduino and get the haxophone to change the sound for example.
The arduino should display the current soundfont used. 

The programs are written in Rust (so the logic can be integrated into the haxophone software)

### Step 1: Pi Hello World
Used to set up the toolchain for cross compile

### Step 2: Pi Serial Test
Used to test the serial connection between haxophone and pc

### Step 3: Pi Haxo Service for Switching Instruments
In this project we create some scripts and different haxo.service config files. When we restart the haxo service with the file for flute for example, then the haxophone uses the flute sound and plays in C.

### Step 4: Pi Haxo Service for Switching Instruments over Serial
Using a modified version of step 2 we restart the haxophone service with another haxo.service config file, when a command is sent via serial

### Next steps
My next steps are:

- On Arduino create a program that sends some strings to serial when buttons are pushed
- On Arduino create a program that displays information received via serial. The display could be LEDs or a small display
 
Connect the dots: 
 
- create a program on Arduino that reads serial to display information, and writes to serial when a button is pushed
- create a program on pi  that reads information from serial, does something and sends the answer back

Integrate into haxophone software

- define the api between pi zero and arduino
- modify the Arduino program so that it does some matching stuff for the haxophone
- integrate the pi program into the haxophone software
