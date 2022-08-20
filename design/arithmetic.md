# arithemtic and bitwise instructions

## add
simple addition

`1100_0100 [src [dest]]`
performs src + dest + carry and stores in src

sets overflow flag if the top bit changes
sets carry flag if the value carried out of the bit width

## sub
simple subtraction

`1100_0101 [src [dest]]`
performs src + !dest + carry and stores in src

sets overflow flag if the top bit changes
sets carry flag if the value carried out of the bit width


## mul and imul
unsigned/signed multiplication

`1100_000s [src [dest]]`
performs src * dest and stores the low half in src and the high half in dest
s determines if the calculation is signed

