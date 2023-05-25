#include "communication.hpp"
/**
 * Components (Button, Rotary Encoder, Potentiometer) modelled as 
 * Finite State Machines
*/

// Async button handler class with debouncing (supposing button is pulled up) 
class Button { 
    int idx; 
    uint8_t channel; 
    int state; 
    int lastState;

    unsigned int lastTimePressed;
    unsigned int debounceDelay;

public:
  Button(uint8_t channel = 0, int idx = 0):channel(channel), state(0),debounceDelay(10), idx(idx) {}
  uint8_t getChannel() {return channel;};
  int getState() {return state;};
  int* getStateRef() {return &state;};

  void nextState(int reading); //Sets state based on input
};

class RotEnc {
  int idx;
  uint8_t Achannel, Bchannel;
  unsigned int time;
  int delay;
  enum State {CW,CCW,RDY};
  State state; // CLOCKWISE COUNTERCLOCKWISE, READY
  int stateA, stateB;
public:
  RotEnc(uint8_t Achannel=0, uint8_t Bchannel=0,int idx = 0):Achannel(Achannel), Bchannel(Bchannel), state(RDY), time(0), delay(50),idx(idx) {}
  uint8_t getChannelA() {return Achannel;};
  uint8_t getChannelB() {return Bchannel;};
  int* getStateRefA() {return &stateA;};
  int* getStateRefB() {return &stateB;};

  void nextState(int A, int B) { // A,B: reading of A,B ch
    if(A != stateA) {
      if(A == B && (state == CW || state == RDY)) {
        state = CW;
        time = millis();
        Message(ROT, idx, increment);
      } else if (A != B && (state == CCW || state == RDY)) {
        state = CCW;
        time = millis();
        Message(ROT, idx, decrement);
      }
      stateA = A;
    }

    if(millis() > time + delay) {
      time+=delay; 
      state = RDY;
    }
  }

};

class Pot {
  int idx;
  uint8_t channel;
  int state; // Pot value filtered with EMA
  int lastState = 0; // Pot value

  float a = 0.2; // alpha value for  EMA (0.3)
public: 
  Pot(uint8_t channel = 0, int idx = 0):channel(channel), state(0),idx(idx) {};
  int* getStateRef() {return &state;}
 
  int to7Bit() const {return state/8; } 

  //Exponential Moving Average (EMA)
  void nextState(int reading) {
    state = (a * reading) + ((1-a) * state);
    if(state/8 != lastState/8) {
      Message(POT, idx, rotate, state) ;
    }
    lastState = state;
  }
};

void Button::nextState(int reading) {
  if(reading != lastState) {
    lastTimePressed = millis();
  }
  if(millis() - lastTimePressed > debounceDelay ) {
    if(reading != state) {
      state = reading;

      if(state == LOW) {
        //Triggers on button down
        lastTimePressed = millis();
        Message(BTN, idx, press);
      } else  {
        Message(BTN, idx, release);
      }
    } else { // No state change
    }
  } 
  lastState = reading;
}