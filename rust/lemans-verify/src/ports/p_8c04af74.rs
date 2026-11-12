//! 0x8c04af74 — free a square 2-D table, element by element, then the rows,
//! then the table descriptor itself.
//!
//! `r4` points at a descriptor: `+0` row-table pointer, `+4` the dimension
//! (used for both the row and the column count), `+8` a field cleared last.
//! Every element slot is a pointer; on the diagonal its target's `+16` field
//! is a nested allocation that is freed first and then zeroed:
//!
//! ```text
//! if (*desc[1] == 0) goto clear;              ; bt 0x8c04aff8
//! for (i = 0; *desc[1] > i; i++) {
//!   for (j = 0; *desc[1] > j; j++) {
//!     if (i == j) {                           ; cmp/eq r12,r13
//!       elem = desc->rows[i][j];
//!       free(elem->nested);                   ; free thunk 0x8c01032c
//!       elem->nested = 0;
//!     }
//!     free(desc->rows[i][j]);                 ; free thunk 0x8c01032c
//!     desc->rows[i][j] = 0;
//!   }
//!   free(desc->rows[i]);                     ; free thunk 0x8c0102cc
//!   desc->rows[i] = 0;
//! }
//! free(desc->rows);                          ; free thunk 0x8c0102cc
//! desc->rows = 0;
//! clear:
//! desc->[2] = 0; desc->[1] = 0;
//! ```
//!
//! The count is re-read from memory on every loop test. Both frees are
//! `jsr @rN` calls whose delay slot loads the argument from the row table, so
//! `pr` and `r4` are visible in the residue; `r14` holds the element pointer
//! across the nested `free`. The thunks tail-jump to `free`, whose null path
//! leaves `r0`/`r1` at the loaded literal and `T` set, so the final `T` is
//! `free`'s last comparison.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c04af74..0x8c04af80: push r14, r13, r12, r11, r10, r9 and pr.
    for reg in [14usize, 13, 12, 11, 10, 9] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let value = cpu.r[reg];
        let _ = mem.write_u32(cpu.r[15], value);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c04af82: mov r4,r9
    cpu.r[9] = cpu.r[4];
    // 0x8c04af84: mov.l @(4,r9),r0
    cpu.r[0] = mem.read_u32(cpu.r[9].wrapping_add(4)).unwrap_or(0);
    // 0x8c04af86: tst r0,r0 ; 0x8c04af88: bt 0x8c04aff8
    let empty = cpu.r[0] == 0;
    cpu.set_t(empty);

    if !empty {
        // 0x8c04af8a: mov #0,r13
        // 0x8c04af8c: bra 0x8c04afe8
        // 0x8c04af8e: mov r13,r10 (delay slot)
        cpu.r[13] = 0;
        cpu.r[10] = cpu.r[13];
        loop {
            // 0x8c04afe8: mov.l @(4,r9),r0
            // 0x8c04afea: cmp/hi r13,r0
            // 0x8c04afec: bt 0x8c04af90
            cpu.r[0] = mem.read_u32(cpu.r[9].wrapping_add(4)).unwrap_or(0);
            let rows = cpu.r[0] > cpu.r[13];
            cpu.set_t(rows);
            if !rows {
                break;
            }
            // 0x8c04af90: mov #0,r12
            // 0x8c04af92: bra 0x8c04afcc
            // 0x8c04af94: mov r12,r11 (delay slot)
            cpu.r[12] = 0;
            cpu.r[11] = cpu.r[12];
            loop {
                // 0x8c04afcc: mov.l @(4,r9),r0
                // 0x8c04afce: cmp/hi r12,r0
                // 0x8c04afd0: bt 0x8c04af96
                cpu.r[0] = mem.read_u32(cpu.r[9].wrapping_add(4)).unwrap_or(0);
                let columns = cpu.r[0] > cpu.r[12];
                cpu.set_t(columns);
                if !columns {
                    break;
                }
                // 0x8c04af96: cmp/eq r12,r13 ; 0x8c04af98: bf 0x8c04afae
                let diagonal = cpu.r[13] == cpu.r[12];
                cpu.set_t(diagonal);
                if diagonal {
                    // 0x8c04af9a: mov.l @r9,r3 ; 0x8c04af9c: mov r10,r0
                    cpu.r[3] = mem.read_u32(cpu.r[9]).unwrap_or(0);
                    cpu.r[0] = cpu.r[10];
                    // 0x8c04af9e: mov.l 0x8c04b010,r1 (0x8c01032c)
                    cpu.r[1] = mem.read_u32(0x8c04b010).unwrap_or(0x8c01032c);
                    // 0x8c04afa0: mov.l @(r0,r3),r3 ; 0x8c04afa2: mov r11,r0
                    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
                    cpu.r[0] = cpu.r[11];
                    // 0x8c04afa4: mov.l @(r0,r3),r14
                    cpu.r[14] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
                    // 0x8c04afa6: jsr @r1 ; 0x8c04afa8: mov.l @(16,r14),r4 (delay slot)
                    cpu.pr = 0x8c04afaa;
                    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(16)).unwrap_or(0);
                    crate::registry::p_8c01032c::run(cpu, mem);
                    // 0x8c04afaa: mov #0,r0 ; 0x8c04afac: mov.l r0,@(16,r14)
                    cpu.r[0] = 0;
                    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[0]);
                }
                // 0x8c04afae: mov.l @r9,r3 ; 0x8c04afb0: mov r10,r0
                cpu.r[3] = mem.read_u32(cpu.r[9]).unwrap_or(0);
                cpu.r[0] = cpu.r[10];
                // 0x8c04afb2: mov.l 0x8c04b010,r1 (0x8c01032c)
                cpu.r[1] = mem.read_u32(0x8c04b010).unwrap_or(0x8c01032c);
                // 0x8c04afb4: mov.l @(r0,r3),r3 ; 0x8c04afb6: mov r11,r0
                cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
                cpu.r[0] = cpu.r[11];
                // 0x8c04afb8: jsr @r1 ; 0x8c04afba: mov.l @(r0,r3),r4 (delay slot)
                cpu.pr = 0x8c04afbc;
                cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
                crate::registry::p_8c01032c::run(cpu, mem);
                // 0x8c04afbc: mov.l @r9,r3 ; 0x8c04afbe: mov r10,r0
                // 0x8c04afc0: mov #0,r1 ; 0x8c04afc2: mov.l @(r0,r3),r3
                // 0x8c04afc4: add #1,r12 ; 0x8c04afc6: mov r11,r0
                // 0x8c04afc8: mov.l r1,@(r0,r3) ; 0x8c04afca: add #4,r11
                cpu.r[3] = mem.read_u32(cpu.r[9]).unwrap_or(0);
                cpu.r[0] = cpu.r[10];
                cpu.r[1] = 0;
                cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
                cpu.r[12] = cpu.r[12].wrapping_add(1);
                cpu.r[0] = cpu.r[11];
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1]);
                cpu.r[11] = cpu.r[11].wrapping_add(4);
            }
            // 0x8c04afd2: mov.l @r9,r3 ; 0x8c04afd4: mov r10,r0
            cpu.r[3] = mem.read_u32(cpu.r[9]).unwrap_or(0);
            cpu.r[0] = cpu.r[10];
            // 0x8c04afd6: mov.l 0x8c04b014,r1 (0x8c0102cc)
            cpu.r[1] = mem.read_u32(0x8c04b014).unwrap_or(0x8c0102cc);
            // 0x8c04afd8: jsr @r1 ; 0x8c04afda: mov.l @(r0,r3),r4 (delay slot)
            cpu.pr = 0x8c04afdc;
            cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
            crate::registry::p_8c0102cc::run(cpu, mem);
            // 0x8c04afdc: mov.l @r9,r3 ; 0x8c04afde: mov r10,r0
            // 0x8c04afe0: mov #0,r1 ; 0x8c04afe2: mov.l r1,@(r0,r3)
            // 0x8c04afe4: add #4,r10 ; 0x8c04afe6: add #1,r13
            cpu.r[3] = mem.read_u32(cpu.r[9]).unwrap_or(0);
            cpu.r[0] = cpu.r[10];
            cpu.r[1] = 0;
            let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1]);
            cpu.r[10] = cpu.r[10].wrapping_add(4);
            cpu.r[13] = cpu.r[13].wrapping_add(1);
        }
        // 0x8c04afee: mov.l 0x8c04b014,r0 (0x8c0102cc)
        // 0x8c04aff0: jsr @r0 ; 0x8c04aff2: mov.l @r9,r4 (delay slot)
        cpu.r[0] = mem.read_u32(0x8c04b014).unwrap_or(0x8c0102cc);
        cpu.pr = 0x8c04aff4;
        cpu.r[4] = mem.read_u32(cpu.r[9]).unwrap_or(0);
        crate::registry::p_8c0102cc::run(cpu, mem);
        // 0x8c04aff4: mov #0,r0 ; 0x8c04aff6: mov.l r0,@r9
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[9], cpu.r[0]);
    }

    // 0x8c04aff8: mov #0,r0
    // 0x8c04affa: mov.l r0,@(8,r9)
    // 0x8c04affc: mov.l r0,@(4,r9)
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[9].wrapping_add(8), cpu.r[0]);
    let _ = mem.write_u32(cpu.r[9].wrapping_add(4), cpu.r[0]);

    // 0x8c04affe..0x8c04b00a: lds.l @r15+,pr ; pop r9..r13 ; rts
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [9usize, 10, 11, 12, 13] {
        cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    // 0x8c04b00c: mov.l @r15+,r14 (delay slot of `rts`)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
