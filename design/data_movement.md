TODO: stack

## mov
internal movement only

`1xxx_xxre [src [dest]]`
e is extension mode, used when moving to a bigger register
    when clear, zero extend
    when set, sign extend
r is reverse
    when clear, copy from src to dest
    when set, copy from dest to src
    allows moving to a to get optimised slightly

if operands are left unspecified, %a is used


## swpr
atomically swap data internally

`1xxx_xxxx [src [dest]]`

src and dest must be the same size
if operands are left unspecified, %a is used
0x85 is `swpr %a, %a`, ie. NOP


## ld
load from memory into a register

`1xxx_aoss dest addr [offset]`
ss is segment selector
a is constant address
    when clear, use a register for address
    when set, use a 16 bit value from memory
o is constant offset
    as above

value size is determined by reg size


## const
load a constant into a register

`1xxx_xxxx [dest [val]]`
if dest is not present, %a is used
if val is not present, 0 is used


## st
store from a register into memory

`1xxx_aoss src addr [offset]`
ss is segment selector
a is constant address
    when clear, use a register for address
    when set, use a 16 bit value from memory
o is constant offset
    as above

value size is determined by reg size


## swpm
atomically swap data in a register and a memory location

`1xxx_aoss src addr [offset]`
ss is segment selector
a is constant address
    when clear, use a register for address
    when set, use a 16 bit value from memory
o is constant offset
    as above

value size is determined by reg size

