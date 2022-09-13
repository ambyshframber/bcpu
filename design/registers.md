# registers  
  
GPRs  
- a  
- ah  
- al  
- b  
- bh  
- bl  
- c  
- cl  
- d  
- dl  
  
offsets (16 bit)  
- co (code offset)  
- do (data offset)  
- eo (extra data offset)  
- so (stack offset)  
  
pointers  
- sp (stack pointer)  
- bp (base/frame pointer)  
- si (source index)  
- di (destination index)  
- rp (return pointer)  
- ro (return offset)  
  
special purpose  
- pc  
- idtp  
- idtl  
- flags  

pseudo-consts  
- byte
- word

## pseudo-consts

retrieves a value from the text  
`opcode byte VAL` or `opcode word VAL VAL`  
allows orthogonal constants
  
  
## flags register  
  
16 bit  
contains  
- carry (0)  
- negative (1)  
- overflow (2)
- zero (3)
- test (4)  
- dseg active (5)  
- privilege (6 and 7)  
- mode32 (bc32 only) (8)  

### privilege levels

- 0: system mode
- 1: reserved
- 2: user mode
- 3: reserved
  
  
# register encoding  
  
operand values are 7 bit  
  
  
  