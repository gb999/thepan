#include "fsm.hpp"
#include "mux.hpp"
#include "communication.hpp"
#include <serial-readline.h>

void received(char*);
SerialLineReader reader(Serial, received); 


int AReadings[16] = {0};
int DReadings[16] = {0};


Button buttons[6];
uint8_t buttonChannels[] = {12,9,6,3,2,15};

Pot pots[5];
uint8_t potChannels[] = {4,3,2,1,0}; 

RotEnc encoders[5];
uint8_t encChannelsA[] = {13, 10, 8, 5, 1};
uint8_t encChannelsB[] = {14, 11, 7, 4, 0};

bool connected = false;

void setup() {
  Serial.begin(115200);
  setupMux();

  for(int i = 0; i < 6; i++) 
    buttons[i] = Button(buttonChannels[i], i);
  
  for(int i = 0; i < 5; i++) 
    pots[i] = Pot(potChannels[i],i);
  

  for(int i = 0; i < 5; i++) 
    encoders[i] = RotEnc(encChannelsA[i], encChannelsB[i],i);

}



void loop() {
  reader.poll();

  if (connected) {
  // Read all multiplexer channels 
    for(int i = 0; i < 16; i++) {
      int j = ((i % 2) * (i + 8) + (i+1) % 2 * i)%16 ; //Keeps a distance of 7 pins between readings
      mux4Read(j, &DReadings[j], &AReadings[j]);
    }

    // Update state of all components
    for(int i = 0; i < 6; i++) {
      buttons[i].nextState(DReadings[buttonChannels[i]]);
    }
    for(int i = 0; i < 5; i++) {
      pots[i].nextState(AReadings[potChannels[i]]);
    }
    for(int i = 0; i < 5; i++) {
      encoders[i].nextState(DReadings[encChannelsA[i]], DReadings[encChannelsB[i]]);
    } 
  } 
}


void received(char* msg) {
  if (String(msg) == "Who are you?") {
    Serial.println("I am The Pan");
    connected = true;
  } else if(String(msg) == "Disconnect") {
    connected = false;
  }
}