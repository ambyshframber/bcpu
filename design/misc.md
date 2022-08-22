# extras  
  
## test  
used to perform checks on integer values  
  
`1111_0000`  
if the next instruction is an arithmetic instruction, run it but do not store the result back to a register.  
flags are modified.  
if the next instruction is not arithmetic, throw an illegal instruction interrupt.  
  
## int  
trigger an interrupt  
  
`1111_000c [val]`  
  
  
# opcode map  
  
https://docs.google.com/spreadsheets/d/e/2PACX-1vQ74tlgjMjUNM8zTx1OTdY4Q1od4owBzQ3g2ICv0DEcSNfWgsrC4BhHiVXj6pMfbzonyQ7JOLEvdooe/pubhtml  
  