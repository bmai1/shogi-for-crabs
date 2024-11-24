const int SW_pin = 2; // switch pin
const int X_pin = A0; 
const int Y_pin = A1; 

void setup() {
  pinMode(SW_pin, INPUT);
  digitalWrite(SW_pin, HIGH);
  Serial.begin(9600);
}

void loop() {
  Serial.print(digitalRead(SW_pin));
  Serial.print("\n");
  Serial.print(analogRead(X_pin));
  Serial.print("\n");
  Serial.println(analogRead(Y_pin));
  Serial.print("\n\n");
  delay(100);
}
