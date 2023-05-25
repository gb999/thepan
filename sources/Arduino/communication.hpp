#ifndef COMMUNICATION_H
#define COMMUNICATION_H

enum GESTURES {
  press,
  release,
  increment,
  decrement,
  rotate
};

enum Component {
  BTN,
  ROT,
  POT
};

/**
 * Sends state changes of components through the Serial port
*/
class Message {
  GESTURES gesture;
  Component component; 
  int id;
  int state;
public:
  static const char * const componentNames[]; 
  static const char * const gestureNames[];
  void send() {
    Serial.print(componentNames[component]);
    Serial.print(" ");
    Serial.print(id);
    Serial.print(" ");
    Serial.print(gestureNames[gesture]);
    Serial.print(" ");
    Serial.println(state);
  };
  Message(Component component, int id, GESTURES gesture, int state=0):gesture(gesture),component(component), id(id), state(state) {
    send();
  };
};

const char * const Message::componentNames[] = {"BTN", "ROT", "POT"};
const char * const Message::gestureNames[] = {"prs", "rel", "inc", "dec", "rot"};


#endif // COMMUNICATION_H