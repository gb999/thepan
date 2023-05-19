const {SerialPort, ReadlineParser} = require('serialport');
const midi = require('midi');
const {Button, RotaryEncoder} = require("./sources/components.js")

const port = "COM9";
const BAUDRATE = 74880;

const midiout = new midi.Output();
const midiin = new midi.Input();

let INPORT = 0;
let OUTPORT = 0;
 
let portCount = midiout.getPortCount();
for(let i = 0; i < portCount; i++) {

    if(midiout.getPortName(i) == "The Pan") {
        OUTPORT = i;
        break;
    }
}

portCount = midiin.getPortCount();
for(let i = 0; i < portCount; i++) {
    if(midiin.getPortName(i) == "The Pan") {
        INPORT = i;
        break;
    }
}

midiout.openPort(OUTPORT);
midiin.openPort(INPORT)

Button.alt = new Button(5);
let encoders = [];
for(let i = 0; i<5;i++) {
    encoders.push(new RotaryEncoder(i));
}
const ALT_BUTTON_INDEX = 5;

const serialport = new SerialPort( {path: port, baudRate: BAUDRATE});
const parser = new ReadlineParser();
serialport.pipe(parser)


const handler = (dt) => {
    //dt: BTN/ROT/POT <IDX> <GESTURE> <STATE>
    let arr = dt.trim().split(/\s/);
    //console.log(arr);
    switch (arr[0]) {
        case "BTN":
            if(arr[1] == ALT_BUTTON_INDEX) {
                Button.alt.nextState(arr[2]);
            } else
            encoders[arr[1]].button.nextState(arr[2])
            //midiout.sendMessage([arr[2] == 0 ? 0x90 : 0x80, arr[1], 127])
        break;
        case "ROT": 
            //midiout.sendMessage([0xB0, parseInt(arr[1])+20, (arr[2]%128+128)%128]);
            encoders[arr[1]].nextState(arr[2], midiout);
        break;

        case "POT":
            midiout.sendMessage([0xB0,parseInt(arr[1])+25,parseInt((1023-parseInt(arr[3]))/8)])
        
        break;
        default:
    }
}

serialport.on("open", () => {
    console.log("Connection open")
})
    
parser.on("data", (dt) => {
    // console.log(dt);    
    handler(dt);

})

