# jumps


## jmp
unconditional jump

`1000_100c [addr]`
if c is high, addr is a constant value
if addr is not present, %a is used


## jz
jump if zero flag zet

`1000_101c [addr]`
if c is high, addr is a constant value
if addr is not present, %a is used

## jc
jump if carry flag set

`1000_110c [addr]`
if c is high, addr is a constant value
if addr is not present, %a is used

## jo
jump if overflow flag set

`1000_111c [addr]`
if c is high, addr is a constant value
if addr is not present, %a is used


## call
function call

`1110_001c [addr]`
if c is high, addr is a constant value
if addr is not present, %a is used

pushes the address of the next instruction to the stack, then jumps

