//! 0x8c0bdcd0 — find the record whose string ends with the needle.
//!
//! `r4` is a descriptor with `count = [r4+12]` and a record table at
//! `[r4+16]`; each record is 108 bytes and its pointer at `+68` reaches a
//! small struct whose `+12` field is a C string. For each record the routine
//! compares the needle (`r5`) against the **tail** of that string: the
//! compare starts at `str + strlen(str) - strlen(needle)` and walks forward
//! until it meets a differing byte or the needle's NUL. A record whose tail
//! matches returns its index; if none does, or `count <= 0`, it returns -1.
//!
//! Details that pin the residue:
//! * `cmp/pl`/`bf.s` run the `mov #0,r12` delay slot on both outcomes, so
//!   `r12 = 0` even on the early -1 exit; nothing sets MACH/MACL there.
//! * Every call is a real `jsr`: the delay slot sets `r4`, `pr` holds the
//!   return address, and 0x8c0f8850 / 0x8c0112f4 are the ported callees.
//! * The inner compare's `bt.s` delay slot `add #1,r0` executes whether or
//!   not the bytes match, so both walk pointers advance every iteration.
//! * `extu.b r4,r3` makes `r3` the unsigned needle byte; the mismatch path
//!   leaves `r3 = needle - candidate` (wrapping), the match path leaves 0.
//! * The match exit reaches `tst r3,r3` (T=1); loop exhaustion leaves the
//!   T=0 from `cmp/gt`. `r9..r14` are popped back to their entry values.
//!
//! `mul.l` fills MACH/MACL from `i*108` on every outer iteration, and is the
//! only writer of them on the looping path. The delay-slot literal reads are
//! taken from the image so a patched pool would behave like the original.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 … mov.l r9,@-r15 ; sts.l pr,@-r15
    for reg in [14usize, 13, 12, 11, 10, 9] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[10] = cpu.r[5]; // mov r5,r10
    cpu.r[9] = cpu.r[4]; // mov r4,r9
    cpu.r[11] = mem.read_u32(cpu.r[9].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r9),r11
    cpu.set_t((cpu.r[11] as i32) > 0); // cmp/pl r11
    cpu.r[12] = 0; // delay slot of bf.s: mov #0,r12

    if (cpu.r[11] as i32) <= 0 {
        cpu.r[0] = 0xffff_ffff; // bf.s 0x8c0bdd42 ; mov #-1,r0
    } else {
        loop {
            // mov r12,r1 ; mov #108,r0 ; mul.l r0,r1 ; sts macl,r1
            cpu.r[1] = cpu.r[12];
            cpu.r[0] = 108;
            let product = (cpu.r[1] as i32 as i64) * (cpu.r[0] as i32 as i64);
            cpu.mach = (product >> 32) as u32;
            cpu.macl = product as u32;
            cpu.r[1] = cpu.macl;
            // mov.l @(16,r9),r4 ; mov.l 0x8c0bdd54,r0 ; add r1,r4
            cpu.r[4] = mem.read_u32(cpu.r[9].wrapping_add(16)).unwrap_or(0);
            cpu.r[0] = mem.read_u32(0x8c0bdd54).unwrap_or(0x8c0f8850);
            cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
            // jsr @r0 ; delay slot: add #32,r4
            cpu.pr = 0x8c0bdcfc;
            cpu.r[4] = cpu.r[4].wrapping_add(32);
            crate::registry::p_8c0f8850::run(cpu, mem);
            cpu.r[13] = cpu.r[0]; // mov r0,r13

            // mov.l 0x8c0bdd58,r0 ; jsr @r0 ; delay slot: mov r10,r4
            cpu.r[0] = mem.read_u32(0x8c0bdd58).unwrap_or(0x8c0112f4);
            cpu.pr = 0x8c0bdd04;
            cpu.r[4] = cpu.r[10];
            crate::registry::p_8c0112f4::run(cpu, mem); // strlen(needle)
            cpu.r[14] = cpu.r[0]; // mov r0,r14

            // mov.l 0x8c0bdd58,r0 ; jsr @r0 ; delay slot: mov r13,r4
            cpu.r[0] = mem.read_u32(0x8c0bdd58).unwrap_or(0x8c0112f4);
            cpu.pr = 0x8c0bdd0c;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c0112f4::run(cpu, mem); // strlen(record string)

            // sub r14,r0 ; mov r10,r5 ; bra 0x8c0bdd1e ; delay slot: add r13,r0
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[14]);
            cpu.r[5] = cpu.r[10];
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[13]);

            let mut hit = false;
            loop {
                // mov.b @r5,r4 ; mov.b @r0,r1 ; extu.b r4,r4
                // extu.b r4,r3 ; extu.b r1,r1 ; extu.b r1,r1 ; add #1,r5
                let needle = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
                let candidate = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
                cpu.r[4] = needle;
                cpu.r[1] = candidate;
                cpu.r[3] = needle;
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                // cmp/eq r1,r3 ; bt.s 0x8c0bdd14 ; delay slot: add #1,r0
                cpu.r[0] = cpu.r[0].wrapping_add(1);
                cpu.set_t(candidate == needle);
                if candidate == needle {
                    // extu.b r4,r1 ; tst r1,r1 ; bf 0x8c0bdd1e
                    cpu.r[1] = cpu.r[4];
                    cpu.set_t(cpu.r[1] == 0);
                    if cpu.r[1] == 0 {
                        // bra 0x8c0bdd34 ; delay slot: mov #0,r3
                        cpu.r[3] = 0;
                        // tst r3,r3 ; bra 0x8c0bdd44 ; delay slot: mov r12,r0
                        cpu.set_t(true);
                        cpu.r[0] = cpu.r[12];
                        hit = true;
                        break;
                    }
                } else {
                    // sub r1,r3 ; tst r3,r3
                    cpu.r[3] = cpu.r[3].wrapping_sub(cpu.r[1]);
                    cpu.set_t(cpu.r[3] == 0);
                    if cpu.r[3] == 0 {
                        // bra 0x8c0bdd44 ; delay slot: mov r12,r0
                        cpu.r[0] = cpu.r[12];
                        hit = true;
                        break;
                    }
                    // bf 0x8c0bdd3c — next record
                    break;
                }
            }
            if hit {
                break;
            }

            // add #1,r12 ; cmp/gt r12,r11 ; bt 0x8c0bdcea
            cpu.r[12] = cpu.r[12].wrapping_add(1);
            cpu.set_t((cpu.r[11] as i32) > (cpu.r[12] as i32));
            if (cpu.r[11] as i32) > (cpu.r[12] as i32) {
                continue;
            }
            cpu.r[0] = 0xffff_ffff; // mov #-1,r0
            break;
        }
    }

    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // mov.l @r15+,r9 … mov.l @r15+,r13
    for reg in [9usize, 10, 11, 12, 13] {
        cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    // rts ; delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
