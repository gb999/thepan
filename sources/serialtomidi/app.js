/** Entry point.
 * 
 */

// Load config
const fs = require("fs");
const configJSON = fs.readFileSync("./config.json");
const CONFIG = JSON.parse(configJSON);

// Open serial port specified in config
const {SerialPort, ReadlineParser} = require('serialport');

const serialport = new SerialPort({path: CONFIG.serialport, baudRate: CONFIG.baudrate});
const serialParser = new ReadlineParser();
serialport.pipe(serialParser);

// Listen to serial port
serialport.on("open", ()=> {
    //console.log("Serial port open.");
})

const serialHandler = require("./serialhandler.js");
serialParser.on("data", (message) => {
    serialHandler(message);
})

// Find (virtual) MIDI I/O ports with name specified in config
const midi = require("midi");

const midiin = new midi.Input();
const midiout = new midi.Output();


// Run loopMIDI sync
const { execFile } = require('node:child_process');
const loopMIDI_Process = execFile(CONFIG.loopmidi_location, (error, stdout, stderr) => {
  if (error) {
    throw error;
  }
  console.log(stdout);
}); 


const findPortIndex = (midiio) => {
    const portcount = midiio.getPortCount();
    for(let i = 0; i < portcount; i++) {
        if(midiio.getPortName(i) == CONFIG.midiportname) {
            return i;
        }
    }
    return -1;
}


loopMIDI_Process.on("spawn", ()=> {

    setTimeout(()=>{},3000)
    const MIDIINPORTINDEX = findPortIndex(midiin);
    const MIDIOUTPORTINDEX = findPortIndex(midiout);
    
    
    if(MIDIINPORTINDEX == -1 || MIDIOUTPORTINDEX == -1) throw new Error("Could not find MIDI I/O ports named " + CONFIG.midiportname + "Were they created with loopMIDI?");
    
    // Open MIDI ports
    midiin.openPort(MIDIINPORTINDEX);
    midiout.openPort(MIDIOUTPORTINDEX);
    
    const Pan = require("./pan.js");
    Pan.midiOutStream = midiout;
})


// Listen to MIDI In messages 
/*midiin.on("message", (deltaTime, message) => {
    //console.log(`dt: ${deltaTime}: ${message}`);
})*/

