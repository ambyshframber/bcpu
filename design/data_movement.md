TODO: stack  
  
## mov  
internal movement only  
  
`1000_000e [src [dest]]`  
e is extension mode, used when moving to a bigger register  
    when clear, zero extend  
    when set, sign extend  
  
if operands are left unspecified, %a is used  
  
  
## swr  
atomically swap data internally  
  
`1xxx_xxxx [src [dest]]`  
  
src and dest must be the same size  
if operands are left unspecified, %a is used  
0x85 is `swpr %a, %a`, ie. NOP  
  
  
## ld  
load from memory into a register  
  
`1001_000s dest addr [offset]`  
s is segment selector  
  
value size is determined by reg size  
  
  
## st  
store from a register into memory  
  
`1010_0sao src addr [offset]`  
s is segment selector  
  
value size is determined by reg size  
  
  
## swm  
atomically swap data in a register and a memory location  
  
`1011_0sao src addr [offset]`  
s is segment selector  
  
value size is determined by reg size  
  
  
## push  
push a value to the stack  
  
`1000_0100 [src]`  
value size is determined by reg size  
  
store the value in src at ss:sp  
decrement sp by the width of src in bytes  
  
## pop  
pop a value from the stack  
  
`1000_0100 [dest]`  
value size is determined by reg size  
  
load the value at ss:sp into dest  
increment sp by the width of dest in bytes  
  
  