# arithemtic and bitwise instructions  
  
## add  
simple addition  
  
`1100_010c [addend]`  
performs %a + addend + carry (if c set) and stores in %a  
  
addend defaults to %b  
sets overflow flag if the top bit changes  
sets carry flag if the calculation carried out of the bit width, otherwise clears it  
sets negative flag if high bit is set, otherwise clears it  
sets zero flag if result is zero  
  
## sub  
simple subtraction  
  
`1100_011c [subtrahend]`  
performs %a - subtrahend - !carry (if c set) and stores in a  
  
base and subtrahend default to %a and %b  
dest defaults to base  
sets overflow flag if the top bit changes  
clears carry flag if the calculation borrowed out of the bit width, otherwise sets it  
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
  
`1100_001s [base [dividend]]`  
performs base / dividend and stores the quotient (rounded down) in base and the remainder in dividend  
s determines if the calculation is signed  
throws a divide by zero exception if dividend is zero  
  
sets negative flag if high bit is set, otherwise clears it  
sets zero flag if quotient is zero  
  
  