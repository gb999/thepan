const {Button, RotaryEncoder, Pot} = require("./components.js");
const fs = require("fs");
const {doublepressinterval, longpressinterval} = JSON.parse(fs.readFileSync("./config.json")).timings;


/**
 * Model of the controller.
 * 
 */
class Pan {
    #altButtonIdx = 6;
    constructor(MidiOutStream) {
        this.buttons = [];
        this.encoders = [];
        this.pots = [];
        this.midiOutStream = MidiOutStream;

        this.gestures = [];
        
        for(let i = 0; i < 5; i++) {
            this.buttons.push(new Button(i));
            this.encoders.push(new RotaryEncoder(i));
            this.pots.push(new Pot(i));
        }
        this.buttons.push(new Button(5))
        this.altButton = this.buttons[5]; //Store a reference to the alt button

        this.EventQueue = [];
    }
    setButtonState(idx, input) {
        let res = this.buttons[idx].setState(input); // pressed|released
        this.EventQueue.push(res);
    }
    setEncoderState(idx, input) {
        let res = this.encoders[idx].setState(input); // velocity
        this.EventQueue.push(res)
    }
    setPotState(idx, input) {
        let res = this.pots[idx].setState(input); // value
        this.EventQueue.push(res);
    }

    get altPressed() {
        return this.altButton.pressed;
    }

    #handlePot(event) {
        let i = event.component.idx;
        let data = Math.floor(1024-event.data); 
        
        this.midiOutStream.sendMessage([0xB0, 0x66+4-i, Math.floor(data/8)]); // Data Entry MSB



        event.state = "discarded";
        
    }
    #handleEncoder(event) {
        let i = event.component.idx;
        let button = this.buttons[i];
        

        let msgval = 64 + event.data.velocity*64;


        const CC = 0xB0;
        let CONTROLLERNUM = 0x0E+4-i;
        //const CONTROLLERNUMLSB = 0x14+32+i;

        
        if(button.event == "singlerelease") {
            CONTROLLERNUM +=0;
        }
        
        if(button.event == "doublerelease") {
            CONTROLLERNUM +=5;
            //step mode

            /*if(event.data.velocity>=0.8) //Skip steps if too fast
                return;
            event.data.velocity /= Math.abs(event.data.velocity)*/
            
        }
        
        if(button.event == "singlepress" ) {
            CONTROLLERNUM +=10;
            
            console.log("holdrotate")
        }
        
        
        if(button.event == "doublepress") {
            CONTROLLERNUM+=15
            console.log("doubleholdrotate")
        }
        
        const message = [CC, CONTROLLERNUM, msgval]
        console.log(event.data.velocity)
        
        /*if(this.altPressed) {
            
        }*/

        this.midiOutStream.sendMessage(message)
        event.state = "discarded";
    }

    #handleButton(event) {
        const component = event.component;
        let idx = component.idx < 5 ? 4 - component.idx : component.idx; // Reverse button order 
        let message = [0x0, idx, 0x7F];
        if(component.pressed == 1) {
            message[0] = 0x90; // NOTEON
        }
        else {
            message[0] = 0x80; // NOTEOFF
            
        }
        this.midiOutStream.sendMessage(message);
        event.state = "discarded";
    }
    
    /**
     * Handles interactions between components
     */
    handle() {
        // An event happens when a component changes state
        const currentTime = Date.now();
        for(let i = 0; i < this.EventQueue.length; i++) {
            let event = this.EventQueue[i]; 

            switch (event.component.constructor.name) {
                case "Pot":
                    this.#handlePot(event);
                break;
                case "Button": 
                {
                    this.#handleButton(event);
                    
                    
                }
                break;    
                case "RotaryEncoder":
                    this.#handleEncoder(event);
                     
                    
                break;
                    
                
                default: // This should never execute 
                throw new Error("Unknown component");
                
            }
            if(event.state == "discarded") 
                this.EventQueue.splice(i,1);    
                            
                
        }
    }

}


module.exports = new Pan();