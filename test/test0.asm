.INCLUDE "./inc/m2560def.inc"
; This is a comment
.org 0x200
.cseg
	ldi r16, 0xFF ; This is an inline comment
	sts DDRL, r16
	out DDRB, r16 ;This is comment with no space


#DEFINE test 0

			; This is a shitty comment


	ldi r16, 0b10000000
	sts PORTL, r16
	ldi r16, 0b00000010
	out PORTB, r16

done:	
jmp done
