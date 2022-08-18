# general purpose:
14 registers + 2 immediate (8 and 16)
imm is read only

16b:
a
b
c
x
st
rt

8b:
ah
al
bl
cl
xl
dseg
cseg

# special purpose:
flags
program counter
idtp
idtl

# flags
0: carry
1: overflow
2: privilege
3: dseg active

# segments
segment registers are shifted left 16 bits and added to addresses, giving theoretical 24 bit addressing
16mb max ram
no seg overlap
cseg adds to program counter
dseg adds to all data addresses
segments are always active in unpriv mode
data segment is controlled by the dseg active bit when in priv mode
segment registers cannot be written to in unpriv mode

# interrupts
an interrupt will cause the program counter and flags register to be pushed to the stack

