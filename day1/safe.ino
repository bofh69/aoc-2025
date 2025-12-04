#include <LiquidCrystal.h>
#include <Stepper.h>

// The display also needs some more signals, see the LCD example
// in Elegoo's documentation.
static const int rs = 12, en = 11, d4 = 5, d5 = 4, d6 = 3, d7 = 7;
static LiquidCrystal lcd(rs, en, d4, d5, d6, d7);

static const int stepsPerRev = 2048;

static Stepper stepper(stepsPerRev, 6, 8, 13, 2);

static void initLcd() {
  lcd.begin(16, 2);
  lcd.print("Hello");
}

void setup() {
  initLcd();

  stepper.setSpeed(16);
  Serial.begin(115200);
  Serial.setTimeout(1000);
}

void loop() {

  static long dialPos = 50;
  static int nZeros = 0;
  static int nPassed = 0;

  String s = Serial.readStringUntil('\n');
  if (s.length() > 0) {
    int dir = 0;
    if (s[0] == 'L') {
      dir = -1;
    } else if (s[0] == 'R') {
      dir = 1;
    } else {
      lcd.setCursor(0, 2);
      lcd.write("Unknown dir");
      Serial.println("Unknown dir: " + s);
    }
    int oldPos = dialPos;
    if (dir) {
      long val = dir * atoi(s.c_str()+1);

      stepper.step(val * stepsPerRev / 100L);
      
      dialPos = dialPos + val;
      if (dialPos >= 100) {
        nPassed += dialPos / 100;
        dialPos = dialPos % 100;
      } else if (dialPos == 0) {
        nPassed += 1;
      } else if (dialPos < 0) {
        if (oldPos == 0) {
          nPassed -= 1;
        }
        while (dialPos < 0) {
          nPassed += 1;
          dialPos += 100;
        }
        if (dialPos == 0) {
          nPassed += 1;
        }
      }
      if (!dialPos) {
          nZeros++;
      }

      lcd.setCursor(0, 0);
      lcd.write(("Part1: " + String(nZeros)).c_str());
      lcd.setCursor(0, 1);
      lcd.write(("Part2: " + String(nPassed)).c_str());

      Serial.println("Old " + String(oldPos) + ", val=" + String(val) + ". New dial pos: " + String(dialPos) + ". " + String(nZeros) + " zeros. " + String(nPassed) + " passed.");
    }
  }
}

// < 6215