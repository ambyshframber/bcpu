# interrupts

the interrupt descriptor table (or IDT) is pointed to by the idtp register. the length of it is given by the idtl register.
"exceptions" are a type of interrupt triggered automatically by the cpu in response to software faults
the first 16 interrupts are reserved as cpu-triggered
further interrupts can be triggered with the int instruction


## list of cpu-triggered interrupts

- illegal opcode (IOP)
    - 0x00
    - triggered when the cpu tries to execute a byte that is not a valid instruction
- illegal instruction (IIN)
    - 0x01
    - triggered when the cpu executes an instruction that it is in the wrong privilege level for
    - for example, directly accessing a segment or special purpose register when in protected mode
- divide by zero
- hardware IRQ
- hardware NMI
- illegal interrupt (double fault)
