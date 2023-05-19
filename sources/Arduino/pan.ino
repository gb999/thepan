#include "fsm.hpp"
#include "mux.hpp"
#include "communication.hpp"



int AReadings[16] = {0};
int DReadings[16] = {0};


Button buttons[6];
uint8_t buttonChannels[] = {2,3,6,9,12,15};

Pot pots[5];
uint8_t potChannels[] = {0,1,2,3,4,5}; 

RotEnc encoders[5];
uint8_t encChannelsA[] = {1, 5, 8, 10, 13};
uint8_t encChannelsB[] = {0, 4, 7, 11, 14};

void setup() {
  Serial.begin(74880);
  setupMux();

  for(int i = 0; i < 6; i++) 
    buttons[i] = Button(buttonChannels[i], i);
  
  for(int i = 0; i < 5; i++) 
    pots[i] = Pot(potChannels[i],i);
  

  for(int i = 0; i < 5; i++) 
    encoders[i] = RotEnc(encChannelsA[i], encChannelsB[i],i);

}



void loop() {
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