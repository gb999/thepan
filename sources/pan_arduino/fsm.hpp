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
  RotEnc(uint8_t Achannel=0, uint8_t Bchannel=0,int idx = 0):Achannel(Achannel), Bchannel(Bchannel), state(RDY), time(0), delay(25),idx(idx) {}
  uint8_t getChannelA() {return Achannel;};
  uint8_t getChannelB() {return Bchannel;};

  void nextState(int A, int B) { // A,B: reading of A,B ch
    unsigned int currentTime = millis(); 

    if(A != stateA) {
      unsigned int dt = currentTime - time;
      if(A == B && (state == CW || state == RDY)) {
        state = CW;
        if(dt >= delay) {
          Message(ROT, idx, +1);
          time = currentTime;
        }
      } else if (A != B && (state == CCW || state == RDY)) {
        state = CCW;
        if(dt >= delay) {
          Message(ROT, idx, -1);
          time = currentTime;
        }
      }
      stateA = A;
    }
  state = RDY;
  
 

  }
};

class Pot {
  int idx;
  uint8_t channel;
  int state; // Pot value filtered with EMA
  int lastState = 0; // Pot value

  float a = 0.6; // alpha value for  EMA (0.3)
public: 
  Pot(uint8_t channel = 0, int idx = 0):channel(channel), state(0),idx(idx) {};
 
  int to7Bit() const {return state/8; } 

  //Exponential Moving Average (EMA)
  void nextState(int reading) {
    state = (a * reading) + ((1-a) * state);
    if(state/8 != lastState/8) {
      Message(POT, idx, state) ;
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
        Message(BTN, idx, 1);
      } else  {
        Message(BTN, idx, 0);
      }
    } else { // No state change
    }
  } 
  lastState = reading;
}