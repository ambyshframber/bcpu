# arithemtic and bitwise instructions  
  
## add  
simple addition  
  
`1100_0100 [base [addend [dest]]]`  
performs base + addend + carry and stores in dest  
  
dest defaults to base if not present  
base and addend default to %a and %b  
sets overflow flag if the top bit changes  
sets carry flag if the calculation carried out of the bit width  
sets negative flag if high bit is set, otherwise clears it  
sets zero flag if result is zero  
  
## sub  
simple subtraction  
  
`1100_0101 [base [subtrahend [dest]]]`  
performs base + !subtrahend + carry and stores in dest  
  
dest defaults to base if not present  
base and subtrahend default to %a and %b  
sets overflow flag if the top bit changes  
clears carry flag if the calculation borrowed out of the bit width  
sets negative flag if high bit is set, otherwise clears it  
sets zero flag if result is zero  
  
  
## mul and imul  
unsigned/signed multiplication  
  
`1100_000s [base [multiplicand]]`  
performs base * multiplicand and stores the low half in base and the high half in multiplicand  
s determines if the calculation is signed  
  
clears overflow flag if if high half is all zeroes or all ones, otherwise sets it  
sets negative flag if high bit is set, otherwise clears it  
sets zero flag if result is zero  
  
## div and idiv  
unsigned/signed division  
  
`1100_000s [base [dividend]]`  
performs base / dividend and stores the result (rounded down) in base and the remainder in dividend  
s determines if the calculation is signed  
throws a divide by zero exception if dividend is zero  
  
sets negative flag if high bit is set, otherwise clears it  
sets zero flag if result is zero  
  
  