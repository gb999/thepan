const fs = require("fs");
const {doublepressinterval, longpressinterval} = JSON.parse(fs.readFileSync("./config.json")).timings;

class Event {
    constructor(component, data = undefined, name = undefined)  {
        this.component = component;
        this.time = Date.now();
        this.data = data; 
        this.name = name;
        this.state = "pending"; // "pending" | "done" | "ignored" (occurs when an other Event modifies the meaning)
    }

}

class Button {
    
    constructor(idx) {
        this.idx = idx;
        this.pressed = 0;   
        this.lastTimePressed = 0;     
        this.lastTimeReleased = 0;     

        this.event = "singlerelease";
    }

    setState(data) {
        const currentTime = Date.now();
        let name = "";
        let dt = currentTime - this.lastTimePressed;
        if(this.pressed == 0) { // Button was not pressed
            if(data == 1) { // Button is pressed now
                // Single or double press
                if(dt > doublepressinterval) 
                {
                    name = "singlepressp";
                    this.event = "singlepress";
                } else if (currentTime - this.lastTimeReleased < longpressinterval){
                    name = "doublepressp";
                    this.event = "doublepress";
                }
                this.lastTimePressed = currentTime;          
            }
        } else { //Button was pressed
            if(data == 0) {
                // Button was released 
                if(dt < longpressinterval && currentTime - this.lastTimeReleased < doublepressinterval ) {
                    name = "doublepressr";
                    this.event = "doublerelease";

                } else {
                    name = "singlepressr";
                    this.event = "singlerelease";
                }
                this.lastTimeReleased = currentTime;
            }
        }
        this.pressed = data;
        return new Event(this, this.pressed, name);
    }
}

class RotaryEncoder {
    constructor(idx) {
        this.idx = idx;
        this.counter = 0;
        this.velocity = 0;
        this.lastEventTime = 0;
 
    }
    setState(data) {
        this.counter += data;
        let currentTime = Date.now();
        let dt = currentTime - this.lastEventTime;
        this.velocity = Math.max(-1, Math.min(data / (1 + dt), 1)); // (1+dt => no division by 0)
        this.lastEventTime = currentTime;
        
        return new Event(this, {velocity: this.velocity});
        
    }
}

class Pot {
    constructor(idx) {
        this.idx = idx;
        this.value = 0;
    }
    setState(data) {
        this.value = data;
        return new Event(this, data);
    }

}

module.exports = {Button, RotaryEncoder, Pot};