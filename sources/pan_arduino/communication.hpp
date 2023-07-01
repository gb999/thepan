#ifndef COMMUNICATION_H
#define COMMUNICATION_H

enum Component {
  BTN,
  ROT,
  POT
};

/**
 * Sends state changes of components through the Serial port
*/
class Message {
  Component component; 
  int id;
  int state;
public:
  static const char * const componentNames[]; 
  void send() {
    Serial.print(componentNames[component]);
    Serial.print(" ");
    Serial.print(id);
    Serial.print(" ");
    Serial.println(state);
  };
  Message(Component component, int id, int state=0):component(component), id(id), state(state) {
    send();
  };
};

const char * const Message::componentNames[] = {"BTN", "ROT", "POT"};


#endif // COMMUNICATION_H