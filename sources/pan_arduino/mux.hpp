#define S0 D6
#define S1 D5
#define S2 D4
#define S3 D3
#define SEL \
  (int[]) { \
    S0, S1, S2, S3 \
  }
#define SIG D1

/**
 * Sets up multiplexer pins
*/
void setupMux() {
  for(int i = 0; i < 4; i++) {
    pinMode(SEL[i], OUTPUT);
  }
  pinMode(A0, INPUT);
  pinMode(SIG, INPUT_PULLUP);
}
/** 
 * Read a 4 channel multiplexer
  * @param ch Channel number to read (in decimal).
  * @param a Pointer to analog output.
  * @param d Pointer to digital output.
*/
void mux4Read(int ch, int* d = NULL, int* a = NULL) {
  //convert decimal channel number to binary SEL signals
  for (int i = 0; i < 4; i++) {
    digitalWrite(SEL[i], ch % 2);
    ch /= 2;
  }
  delayMicroseconds(2); //multiplexer delay 
  if(d) *d = digitalRead(SIG); 
  if(a) *a = analogRead(A0);
  
}