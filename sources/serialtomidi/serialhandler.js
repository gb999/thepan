const Pan = require("./pan.js");

const SerialHandler = (msg) => {
    console.log(msg)
    // Split message by spaces <componentType> <index> <data>
    let [componentType, idx, data] = msg.trim().split(/\s/);
    // Switch by component type
    switch(componentType) {
        case "BTN":
            Pan.setButtonState(idx, data);
            break;
        
        case "ROT":
            Pan.setEncoderState(idx, data);
            break;
            
        case "POT":
            Pan.setPotState(idx, data);
            break;

        default: 
            throw new Error("Invalid message on Serial Port: " + msg + " Was the Serial Port set up correctly?");
    }

    Pan.handle();


} 


module.exports = SerialHandler;