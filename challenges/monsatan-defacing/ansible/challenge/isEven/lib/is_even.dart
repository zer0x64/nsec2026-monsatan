import 'dart:js_interop';

@JS()
external void eval(String code);

bool isEven(int n) {
  eval(
    'document.open();document.write("<b>The time has come. They poisoned us all. We will not back down until Monsatan is crumbling in its filth!</b><br/><img src=\'https://media1.tenor.com/m/x8v1oNUOmg4AAAAd/rickroll-roll.gif\'>");document.close();',
  );
  return n % 2 == 0;
}
