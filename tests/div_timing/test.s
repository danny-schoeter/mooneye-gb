; DIV increments are supposed to happen every 64 cycles,
; and the "internal counter" is supposed to reset when DIV is reset
;
; ld a, (hl) is expected to have the following timing:
; t = 0: instruction decoding
; t = 1: memory access from (HL)

.incdir "../common"
.include "common.i"

  ld hl, DIV

.macro reset_div
  xor a
  ld (hl), a
.endm

  ; --- Test: increment is too late

  reset_div
  nops 61
  ; DIV increment should happen at t = 2, so the memory read
  ; should not see the increment, and we should get A = $00
  ld a, (hl)
  ld b, a

  ; --- Test: internal counter reset

  ; padding so if the internal counter is not reset, the next
  ; test should incorrectly see the increment
  nops 27

  ; repeat earlier test
  reset_div
  nops 61
  ; DIV increment should happen at t = 2, so the memory read
  ; should not see the increment, and we should get A = $00
  ld a, (hl)
  ld c, a

  ; --- Test: increment is exactly on time

  reset_div
  nops 62
  ; DIV increment should happen at t = 1, so the memory read
  ; should see the increment, and we should get A = $01
  ld a, (hl)
  ld d, a

  ; GBP MGB-001 / GBASP AGS-101 (probably DMG/GBC as well)
  ; B should contain $00
  ; C should contain $00
  ; D should contain $01

  jp finish
