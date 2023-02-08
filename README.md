# Problem 0: A computer
 *Problem* 
Below is the instruction set for a machine:

0x00   halt -- Terminate program

0x01   nop  -- Do nothing

0x02   li   -- Load Immediate: li R1 0x00000000
               Load 0x00000000 into R1

0x03   lw   -- Load Word: lw R1 R2
               Load the contents of the memory location
               pointed to by R2 into R1

0x04   sw   -- Store Word: sw R1 R2
               Store the contents of R2 in the memory
               location pointed to by R1

0x05   add  -- Add: add R3 R1 R2
               Add R1 to R2 and store the result in R3

0x06   sub  -- Subtract: sub R3 R1 R2
               Subtract R2 from R1 and store the result in R3

0x07   mult -- Multiply: mult R3 R1 R2
               Multiply R1 by R2 and store the result in R3

0x08   div  -- Divide: div R3 R1 R2
               Divide R1 by R2 and store the result in R3

0x09   j    -- Unconditional Jump: j 0x00000000
               Jump to memory location 0x00000000

0x0A   jr   -- Unconditional Jump (Register): jr R1
               Jump to memory location stored in R1

0x0B   beq  -- Branch if Equal: bne R1 R2 R3
               Branch to memory location stored in R3
               if R1 and R2 are equal

0x0C   bne  -- Branch if Not Equal: beq R1 R2 R3
               Branch to memory location stored in R3
               if R1 and R2 are not equal

0x0D   inc  -- Increment Register: inc R1
               Increment R1

0x0E   dec  -- Decrement Register: dec R1
               Decrement R1

It has five registers and 64K of memory in a 32-bit address space, that is 0x00000000–0x0000FFFF. The five registers consist of three general purpose (R1, R2, R3); and two special purpose, a program counter (instruction pointer) register (PC), and a conditional register (COND) that stores conditional flags which provide information about the most recently executed calculation allowing programs to check logical conditions.

Each instruction is encoded in a half word (16 bits) in little endian. The first 4 bits (half byte) contain the instruction number, which can be anything from 0x0 to 0xE, while the second, third and fourth half-bytes (4 bit sections) contain register numbers. (Note: For some instructions like li the remainder of the 8 bits after the instruction number and the register number contain an 8 bit immediate value.)

    Write an assembler for the instruction set that takes a text assembly program written for the above instruction set and produces the program as a set of 16-bit numbers. Basically, the task is to encode the text of the assembly into the bytecode format.

    Write a simulator for the machine that will take the output of the assembler and execute it, correctly. For example, you should be able to run the following program:

    ; a simple counter program.
    li R1 0x00000000
    ; end
    li R2 0x0000FFFF
    ; memory location of loop start
    li R3 loop
    loop:
      ; store the contents of R1 at the memory location pointed by R1
      sw R1 R1
      ; increment the counter
      inc R1
      ; loop if the counter hasn't yet reached the end
      bne R1 R2 R3
      ; end program
      halt

    Programs should be loaded from 0x0000CFFF to the end of memory so that any memory before that is usable by the programmer.
    The simulator should log the register values after every cycle (upon execution of every instruction).


