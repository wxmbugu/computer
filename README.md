 # PROBLEM - 0 : COMPUTER
 -  Problem to this question [problem](https://github.com/wxmbugu/computer/blob/main/PROBLEM.md)
 - This is 32bit computer simulator and assembler. This has been implemented using the [pesapal-arch](https://github.com/wxmbugu/computer/blob/main/assembler/isa.arch).
  - The problem was to build a 16bit computer but so as to fit the address and avoid some bit integers(immediate value and addresses) being stripped off when decoding them to 16bits  to avoid that I added a 16bit (address/immediate value).  
  

 **Prerequites**
  - Rust installed [rust](https://www.rust-lang.org/tools/install)

#### RUN ASSEMBLER
```
$ cd assembler
$ cargo run --bin assembler --  -i test.s -o test.bin         
```

#### RUN SIMULATOR
```
$ cd assembler
$ cargo run --bin computer -- -i test.bin       
```


