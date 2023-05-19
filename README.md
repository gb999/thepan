# The Pan
## From a pan to a MIDI controller
I was cooking pancakes for dinner on a peaceful evening. It was going well, half of the pancakes were done. Then I tried to flip the next pancake...
And suddenly I found the handle of the pan in my hand. **Without the pan!** *Revenge had to be served...*

<img alt="The Pan from below" src="https://github.com/gb999/thepan/assets/48630952/aaa561df-8837-4fc7-982e-e7d50344a3ec" width = "40%">
<img alt="The Pan from above" src="https://github.com/gb999/thepan/assets/48630952/a0822521-735f-47f2-84dc-502bb2e25141" width = "40%"> 

## Hardware
*The Pan* has 5 **potentiometers** and 5 **rotary encoders** (with push buttons) below them, also it has 1 **button** (alt).
The **microcontroller** is a Wemos D1 R2 board. The rotary encoders and the button are connected to a CD74HC4067 16 channel **multiplexer**. The multiplexer is read with a digital pin on the microcontroller. The potentiometers are connected to another multiplexer of the same type. It is read by the analog pin on the microcontroller. It still has 11 channels remaining for future upgrades (e.g. some piezo sensors as drum pads). The select pins of the multiplexers are connected to the same digital pins on the microcontroller. The powering and communication of the device is through the micro-USB port of the board.  

## How does it work?
The [code](./sources/Arduino) on the microcontroller reads the state of the components on *The Pan* and sends a message through the *Serial Port* when a change is detected. A Node.js [script](./sources/serialtomidi) reads these messages, gets the necessary data from the microcontroller to determine what is being done on the pan. (e.g. holding down a button, or pressing it shortly). Then it translates these messages to MIDI messages. These messages are then sent through a virtual MIDI port (the port is created using Tobias Erichsens [loopMIDI](https://www.tobias-erichsen.de/software/loopmidi.html) application)
