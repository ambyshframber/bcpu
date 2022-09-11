# jumps  
  
  
## jmp  
unconditional jump  
  
`1000_1000 [addr]`  
if addr is not present, %a is used  
  
  
## jz  
jump if zero flag set  
  
`1000_1001 [addr]`  
if addr is not present, %a is used  

## jnz  
jump if zero flag clear  
  
`1000_1010 [addr]`  
if addr is not present, %a is used  
  
  
## call  
function call  
  
`1010_100c [addr]`  
if c is high, addr is a constant value  
if addr is not present, %a is used  
  
moves the address of the next instruction to %rp, then jumps  
  
## lcall  
long call  
  
`1010_101c addr segment`  
if c is high, addr and segment are constant values  
  
moves the address of the next instruction to %rp, moves %co to %rop, then jumps  
  
  
## ret  
function return  
  
`1110_0100`  
  
moves %rp to %pc  
  
## lret  
long return  
  
`1110_001`  
  
moves %rp to %pc and moves %rop to %co  
  
  