class Button {
    static doublePressTime = 250;
    
    static DOUBLEPRESSP = 0; 
    static DOUBLEPRESSR = 1; 
    static PRESSP = 2; 
    static PRESSR = 3;
    static HANDLED = -1;
    
    
    constructor(idx) {
        this.idx = idx;
        this.pressed = 0;
        this.lastPressTime = 0;
        this.lastReleaseTime = 0;
        this.event = Button.HANDLED;
    }
    static alt = Button;
    /**
     * 
     * @param {string} s input state 
     */
    nextState(s) {
        let result = -1;
        if(s == "prs") { //PRESS
            if(!this.pressed) {

                if(Date.now() - this.lastPressTime <= Button.doublePressTime) {
                    //Double Press Event Triggered By Press
                    result = Button.DOUBLEPRESSP;
                } else {
                    result = Button.PRESSP;
                }
            }
            this.pressed = true;
            this.lastPressTime = Date.now();
        } else { //RELEASE
            if(this.pressed) {
                if(Date.now() - this.lastPressTime <= Button.doublePressTime 
                && Date.now() - this.lastReleaseTime <= Button.doublePressTime) {
                    //Double Press Event Triggered By Release
                    result = Button.DOUBLEPRESSR;
                } else {
                    //Single Press Event Triggered By Release
                    result = Button.PRESSR;
                }
            }
            this.pressed = false;
            this.lastReleaseTime = Date.now();
        }
        this.event = result;
    }
}

class RotaryEncoder {
    static BANDTYPECHANGE = 0;
    constructor(idx) {
        this.idx = idx;
        this.counter = 0; //
        this.button = new Button(idx);
        this.event;

    }
    nextState(s, midiout) { // s: inc/dec
        // BAND TYPE CHANGE
        if (this.button.event == Button.DOUBLEPRESSR 
            && this.event != RotaryEncoder.BANDTYPECHANGE) {

            this.event = RotaryEncoder.BANDTYPECHANGE;
            this.button.event = Button.HANDLED;
            //console.log("Band type change start")

        } 
        if(this.button.event == Button.DOUBLEPRESSR 
            && this.event == RotaryEncoder.BANDTYPECHANGE ) {
        
            this.event = RotaryEncoder.HANDLED;
            this.button.event = Button.HANDLED;
            // console.log("Band type change end")
        }
        if(this.event == RotaryEncoder.BANDTYPECHANGE) {
            // console.log("Band Type change");
            midiout.sendMessage([0xB0, s == "inc" ? 0x60:0x61, this.idx+15]);
            
        }
        
        if(this.button.event == Button.DOUBLEPRESSP && this.event != RotaryEncoder.BANDTYPECHANGE) {
            // console.log("ORDER")
            midiout.sendMessage([0xB0, s == "inc" ? 0x60:0x61, this.idx+10]);
        }
        
        if(this.button.event != Button.DOUBLEPRESSP && this.button.pressed ) {
            // console.log("BAND WIDTH CHANGE")
            midiout.sendMessage([0xB0, s == "inc" ? 0x60:0x61, this.idx+5]);

        }

        if(Button.alt.pressed && 
            (this.button.event == Button.HANDLED || this.button.event == Button.PRESSR)
            && this.event == RotaryEncoder.HANDLED) {
                
            //HAVE TO IMPLEMENT SMOOTH CHANGE
            midiout.sendMessage([0xB0, s == "inc" ? 0x60:0x61, this.idx]);
            //console.log("SMOOTH")
        }

        if(!Button.alt.pressed && 
            (!this.button.pressed && this.button.event == Button.HANDLED || this.button.event == Button.PRESSR)
             && this.event == RotaryEncoder.HANDLED) {
            // console.log("gain")

            midiout.sendMessage([0xB0, s == "inc" ? 0x60:0x61, this.idx]);

        }

    }
}

module.exports = {
    Button,
    RotaryEncoder
}