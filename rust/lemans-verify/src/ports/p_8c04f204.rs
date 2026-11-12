//! 0x8c04f204 — scan a pointer array for an element matching `param_1`.
//!
//! `param_1` is a three-pointer struct; `param_2` walks an array of element
//! pointers bounded by `count = [[8c2ab468]+0x10]`:
//!
//! ```text
//! while ([r12+8] + [r12+4]*4 > r11) {          ; count*4 end vs cursor
//!   e = *r11
//!   if ((u8)f_8c10e344(e[1]) != 0) {           ; byte at [[e[1]]+82]
//!     a = f_8c10e364(e[1])                    ; [e[1]]+36
//!     b = f_8c10e36c(e[1])                    ; [e[1]]+32
//!     if (f_8c070b0c(param_1[0][3], (*a)[3]) == 0 &&
//!         f_8c070b0c(param_1[2][3], (*b)[3]) == 0) return 1;
//!   }
//!   c = f_8c10e3b4(e[1])                      ; [e[1]]+20
//!   d = f_8c10e3bc(e[1])                      ; [e[1]]+16
//!   if (f_8c070b0c(param_1[0][3], (*c)[3]) == 0 &&
//!       f_8c070b0c(param_1[2][3], (*d)[3]) == 0) return 1;
//!   r11 += 4;
//! }
//! return 0;
//! ```
//!
//! Call sites (nine, each jsr's delay slot folded in):
//! * 0x8c04f220 0x8c10e344 with `r4 = [r13+4]` (delay slot load)
//! * 0x8c04f22c 0x8c10e364 with `r4 = [r13+4]` (delay slot load)
//! * 0x8c04f234 0x8c10e36c with `r4 = [r13+4]` (delay slot load)
//! * 0x8c04f242 0x8c070b0c with `r4 = [[r10]+12]`, `r5 = [[r14]+12]`
//! * 0x8c04f252 0x8c070b0c with `r4 = [[r10+8]+12]`, `r5 = [[r14]+12]`
//! * 0x8c04f260 0x8c10e3b4 with `r4 = [r13+4]` (delay slot load)
//! * 0x8c04f26a 0x8c10e3bc with `r4 = [r13+4]` (reloaded before `r13` is
//!   overwritten; the jsr delay slot itself is a nop)
//! * 0x8c04f278 0x8c070b0c with `r4 = [[r10]+12]`, `r5 = [[r13]+12]`
//! * 0x8c04f288 0x8c070b0c with `r4 = [[r10+8]+12]`, `r5 = [[r14]+12]`
//!
//! Side effects: r0/r1/r3/r4/r5/r10-r14 (plus the callees' own r0/r1/r3-r7
//! leftovers), r15 restored by the six-word push/pop, `pr` saved/restored
//! around the calls, and T from the last comparison: the loop's unsigned
//! `cmp/hi` (false) on the exit-0 path, the last `tst r0,r0` (true) on both
//! return-1 paths. No stores outside the stack window.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: mov.l r14,@-r15 … sts.l pr,@-r15 (six words).
    for reg in [14usize, 13, 12, 11, 10] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[11] = cpu.r[5]; // mov r5,r11 — cursor
    cpu.r[10] = cpu.r[4]; // mov r4,r10 — param_1
    // mov.l 0x8c04f2b4,r0 ; mov.l @r0,r0 ; bra 0x8c04f296
    cpu.r[0] = mem.read_u32(0x8C04F2B4).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[12] = mem.read_u32(cpu.r[0].wrapping_add(0x10)).unwrap_or(0); // delay slot

    'scan: loop {
        // mov.l @(4,r12),r0 ; mov.l @(8,r12),r1 ; shll2 r0 ; add r0,r1 ;
        // cmp/hi r11,r1 ; bt 0x8c04f21c
        cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] = cpu.r[0] << 2; // shll2 r0
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // add r0,r1
        cpu.set_t(cpu.r[1] > cpu.r[11]); // cmp/hi r11,r1 (unsigned)
        if !cpu.t() {
            cpu.r[0] = 0; // mov #0,r0
            break 'scan;
        }

        cpu.r[13] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r13

        // 0x8c04f21e/0x8c04f220/0x8c04f222 — 0x8c10e344(*(r13+4)).
        cpu.r[0] = mem.read_u32(0x8C04F2B8).unwrap_or(0);
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        cpu.pr = 0x8C04F224;
        crate::registry::p_8c10e344::run(cpu, mem);
        cpu.r[0] &= 0xFF; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0

        if !cpu.t() {
            // ---- first pair (0x8c04f22a..0x8c04f258)
            cpu.r[0] = mem.read_u32(0x8C04F2BC).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
            cpu.pr = 0x8C04F230;
            crate::registry::p_8c10e364::run(cpu, mem);
            cpu.r[14] = cpu.r[0]; // mov r0,r14

            cpu.r[0] = mem.read_u32(0x8C04F2C0).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
            cpu.pr = 0x8C04F238;
            crate::registry::p_8c10e36c::run(cpu, mem);

            cpu.r[3] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r3
            cpu.r[1] = mem.read_u32(cpu.r[10]).unwrap_or(0); // mov.l @r10,r1
            cpu.r[14] = cpu.r[0]; // mov r0,r14
            cpu.r[0] = mem.read_u32(0x8C04F2C4).unwrap_or(0);
            cpu.r[5] = mem.read_u32(cpu.r[3].wrapping_add(12)).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap_or(0);
            cpu.pr = 0x8C04F246;
            crate::registry::p_8c070b0c::run(cpu, mem);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0

            if cpu.t() {
                cpu.r[3] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r3
                cpu.r[1] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
                cpu.r[0] = mem.read_u32(0x8C04F2C4).unwrap_or(0);
                cpu.r[5] = mem.read_u32(cpu.r[3].wrapping_add(12)).unwrap_or(0);
                cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap_or(0);
                cpu.pr = 0x8C04F256;
                crate::registry::p_8c070b0c::run(cpu, mem);
                cpu.set_t(cpu.r[0] == 0);
                if cpu.t() {
                    cpu.r[0] = 1; // bra 0x8c04f2a4 delay slot
                    break 'scan;
                }
            }
        }

        // ---- second pair (0x8c04f25e..0x8c04f292)
        cpu.r[0] = mem.read_u32(0x8C04F2C8).unwrap_or(0);
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        cpu.pr = 0x8C04F264;
        crate::registry::p_8c10e3b4::run(cpu, mem);

        // 0x8c04f264 reloads the same argument before r13 is replaced.
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        cpu.r[13] = cpu.r[0]; // mov r0,r13
        cpu.r[0] = mem.read_u32(0x8C04F2CC).unwrap_or(0);
        cpu.pr = 0x8C04F26E;
        crate::registry::p_8c10e3bc::run(cpu, mem);

        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r3
        cpu.r[14] = cpu.r[0]; // mov r0,r14
        cpu.r[1] = mem.read_u32(cpu.r[10]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8C04F2C4).unwrap_or(0);
        cpu.r[5] = mem.read_u32(cpu.r[3].wrapping_add(12)).unwrap_or(0);
        cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap_or(0);
        cpu.pr = 0x8C04F27C;
        crate::registry::p_8c070b0c::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            cpu.r[11] = cpu.r[11].wrapping_add(4); // add #4,r11
            continue 'scan;
        }

        cpu.r[3] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r3
        cpu.r[1] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8C04F2C4).unwrap_or(0);
        cpu.r[5] = mem.read_u32(cpu.r[3].wrapping_add(12)).unwrap_or(0);
        cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap_or(0);
        cpu.pr = 0x8C04F28C;
        crate::registry::p_8c070b0c::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0);
        if cpu.t() {
            cpu.r[0] = 1; // bra 0x8c04f2a4 delay slot
            break 'scan;
        }
        cpu.r[11] = cpu.r[11].wrapping_add(4); // add #4,r11
    }

    // Epilogue: lds.l @r15+,pr ; mov.l @r15+,r10 … r13 ; rts ;
    // mov.l @r15+,r14 (rts delay slot).
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [10usize, 11, 12, 13, 14] {
        cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
