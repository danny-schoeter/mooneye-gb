; This file is part of Mooneye GB.
; Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
;
; Mooneye GB is free software: you can redistribute it and/or modify
; it under the terms of the GNU General Public License as published by
; the Free Software Foundation, either version 3 of the License, or
; (at your option) any later version.
;
; Mooneye GB is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
; GNU General Public License for more details.
;
; You should have received a copy of the GNU General Public License
; along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.

; If bit 5 (mode 2 OAM interrupt) is set, an interrupt is also triggered
; at line 144 when vblank starts.
; This test measures the cycles between vblank<->vblank and compares that to vblank<->stat_m2_144
; Expected behaviour: vblank and stat_m2_144 are triggered at the same time

; Verified results:
;   pass: DMG, MGB
;   fail: CGB, AGB, AGS

.incdir "../../common"
.include "common.s"

.macro halt_until ARGS intr
  xor a
  ld_ff_a IF
  ld a, intr
  ld_ff_a IE

  ei
  halt
  nop

  jp fail_halt
.endm

  ld hl, intr_vec_vblank
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <test_round1
  ld (hl+), a
  ld a, >test_round1
  ld (hl), a

  halt_until INTR_VBLANK

fail_halt:
  test_failure_string "HALT"

test_round1:
  ld hl, intr_vec_vblank
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <finish_round1
  ld (hl+), a
  ld a, >finish_round1
  ld (hl), a

  wait_ly 143

  nops 54
  ld_ff_a DIV

  halt_until INTR_VBLANK

finish_round1:
  ld_a_ff DIV
  ld (round1), a

  ld hl, intr_vec_vblank
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <test_round2
  ld (hl+), a
  ld a, >test_round2
  ld (hl), a

  halt_until INTR_VBLANK

test_round2:
  ld hl, intr_vec_vblank
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <finish_round2
  ld (hl+), a
  ld a, >finish_round2
  ld (hl), a

  wait_ly 143

  nops 55
  ld_ff_a DIV

  halt_until INTR_VBLANK

finish_round2:
  ld_a_ff DIV
  ld (round2), a

  ld hl, intr_vec_vblank
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <test_round3
  ld (hl+), a
  ld a, >test_round3
  ld (hl), a

  ld a, $20
  ld_ff_a STAT

  halt_until INTR_VBLANK

test_round3:
  ld hl, intr_vec_stat
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <finish_round3
  ld (hl+), a
  ld a, >finish_round3
  ld (hl), a

  wait_ly 143

  nops 54
  ld_ff_a DIV

  halt_until INTR_STAT

finish_round3:
  ld_a_ff DIV
  ld (round3), a

  ld hl, intr_vec_vblank
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <test_round4
  ld (hl+), a
  ld a, >test_round4
  ld (hl), a

  halt_until INTR_VBLANK

test_round4:
  ld hl, intr_vec_stat
  ld a, $C3
  ld (hl+), a ; JP nnnn
  ld a, <finish_round4
  ld (hl+), a
  ld a, >finish_round4
  ld (hl), a

  wait_ly 143

  nops 55
  ld_ff_a DIV

  halt_until INTR_STAT

finish_round4:
  ld_a_ff DIV

test_finish:
  ld e, a
  ld a, (round3)
  ld d, a
  ld a, (round2)
  ld c, a
  ld a, (round1)
  ld b, a

  save_results
  assert_b $01
  assert_c $00
  assert_d $01
  assert_e $00
  jp process_results

.org INTR_VEC_VBLANK
  jp intr_vec_vblank

.org INTR_VEC_STAT
  jp intr_vec_stat

.ramsection "Test-State" slot 2
  intr_vec_vblank dsb 3
  intr_vec_stat dsb 3
  round1 db
  round2 db
  round3 db
.ends