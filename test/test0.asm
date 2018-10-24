.INCLUDE "./inc/m2560def.inc"
; This is a comment
.cseg
	ldi r16, 0xFF ; This is an inline comment
	sts DDRL, r16
	out DDRB, r16 ;This is comment with no space



			; This is a shitty comment


	ldi r16, 0b10000000
	sts PORTL, r16
	ldi r16, 0b00000010
	out PORTB, r16

done:	jmp done
