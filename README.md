# AVR Assembler

This project attempts to implement a simple assembler for the AVR Assembly
Language specified [here](https://www.microchip.com/webdoc/avrassembler/index.html).
The general structure and design will be adapted from
[this](http://www.ssmengg.edu.in/weos/weos/upload/EStudyMaterial/Cse/4thSem/new/System%20Software/SS_Assembler%20design.pdf)
paper as well as from the [avra](http://avra.sourceforge.net/) assembler.

The purpose of this project is to learn how an assembler works, as well as to
learn the basics of the Rust programming language. To begin with, the assembler
will only compile code for the ATmega2560 microcontroller, but support for
additional controllers may be implemented in the future.

## Progress

### Back-end
- [x] Symbol table
- [x] Address counter
- [x] Line counter
- [x] Error reporting w/ line number, line printed, and cause
- [x] Comment ignoring
- [ ] Operand counting
- [x] Base conversion (0x, 0b number representations)
- [ ] Underscores in numbers for readability
- [x] Instruction length calculation
- [ ] Instruction assembly to binary
- [ ] Output formats (.hex, .obj, etc)

### Error handling
- [x] No file specified
- [x] File failed to open
- [x] Redefinition of symbol
- [x] Undefined symbol
- [x] Register index out of bounds
- [x] Invalid number format
- [ ] Incorrect number of operands
- [ ] Invalid syntax
- [ ] Instruction not supported on specified hardware

### Directives
- [ ] BYTE
- [ ] CSEG
- [ ] CSEGSIZE
- [ ] DB
- [x] DEF
- [ ] DSEG
- [ ] DW
- [ ] ENDM, ENDMACRO
- [ ] EQU
- [ ] ESEG
- [ ] EXIT
- [ ] INCLUDE
- [ ] LIST
- [ ] LISTMAC
- [ ] MACRO
- [ ] NOLIST
- [x] ORG
- [ ] SET
- [ ] ELSE,ELIF
- [ ] ENDIF
- [ ] ERROR
- [ ] IF,IFDEF,IFNDEF
- [ ] MESSAGE
- [ ] DD
- [ ] DQ
- [ ] UNDEF
- [ ] WARNING
- [ ] OVERLAP/NOOVERLAP

### Preprocessor
- [ ] #define
- [ ] #undef
- [ ] #ifdef
- [ ] #ifndef
- [ ] #if and #elif
- [ ] #else
- [ ] #endif
- [ ] #error, #warning and #message
- [ ] #include
- [ ] #pragma, general purpose
- [ ] #pragma , AVR part related
- [ ] # (empty directive)

### Instructions
- [ ] ADC
- [ ] ADD
- [ ] ADIW
- [ ] AND
- [ ] ANDI
- [ ] ASR
- [ ] BCLR
- [ ] BLD
- [ ] BRBC
- [ ] BRBS
- [ ] BRCC
- [ ] BRCS
- [ ] BREAK
- [ ] BREQ
- [ ] BRGE
- [ ] BRHC
- [ ] BRHS
- [ ] BRID
- [ ] BRIE
- [ ] BRLO
- [ ] BRLT
- [ ] BRMI
- [ ] BRNE
- [ ] BRPL
- [ ] BRSH
- [ ] BRTC
- [ ] BRTS
- [ ] BRVC
- [ ] BRVS
- [ ] BSET
- [ ] BST
- [ ] CALL
- [ ] CBI
- [ ] CBR
- [ ] CLC
- [ ] CLH
- [ ] CLI
- [ ] CLN
- [ ] CLR
- [ ] CLS
- [ ] CLT
- [ ] CLV
- [ ] CLZ
- [ ] COM
- [ ] CP
- [ ] CPC
- [ ] CPI
- [ ] CPSE
- [ ] DEC
- [ ] EICALL
- [ ] EIJMP
- [ ] ELPM
- [ ] EOR
- [ ] FMUL
- [ ] FMULS
- [ ] FMULSU
- [ ] ICALL
- [ ] IJMP
- [ ] IN
- [ ] INC
- [ ] JMP
- [ ] LD
- [ ] LAT
- [ ] LAS
- [ ] LAC
- [ ] LD (LDD)
- [ ] LD (LDD)
- [ ] LDI
- [ ] LDS
- [ ] LDS
- [ ] LPM
- [ ] LSL
- [ ] LSR
- [ ] MOV
- [ ] MOVW
- [ ] MUL
- [ ] MULS
- [ ] MULSU
- [ ] NEG
- [ ] NOP
- [ ] OR
- [ ] ORI
- [ ] OUT
- [ ] POP
- [ ] PUSH
- [ ] RCALL
- [ ] RET
- [ ] RETI
- [ ] RJMP
- [ ] ROL
- [ ] ROR
- [ ] SBC
- [ ] SBCI
- [ ] SBI
- [ ] SBIC
- [ ] SBIS
- [ ] SBIW
- [ ] SBR
- [ ] SBRC
- [ ] SBRS
- [ ] SEC
- [ ] SEH
- [ ] SEI
- [ ] SEN
- [ ] SER
- [ ] SES
- [ ] SET
- [ ] SEV
- [ ] SEZ
- [ ] SLEEP
- [ ] SPM
- [ ] ST
- [ ] ST (STD)
- [ ] ST (STD)
- [ ] STS
- [ ] STS
- [ ] SUB
- [ ] SUBI
- [ ] SWAP
- [ ] TST
- [ ] WDR
- [ ] XCH

## Differences between proprietary AVR Assembler
* Cyclic dependencies forbidden
