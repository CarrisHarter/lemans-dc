//! 0x8c07363c — scan a table of 0x30-byte entries at param_5 ([r15+40] at
//! entry... after the prologue `mov.l @(40,r15),r3`), from index param_4 (r7)
//! to param_6 ([r15+44]); for entries whose tag at +8/+0xc/+0x10 equals
//! uVar4, decrement the count at +0x28 by 2 (flag byte at 0x8c1f8210 != 0)
//! or by 1 (flag == 0 and one of the three u16 pairs matches).
//!
//! uVar4 (r1) is built from the u16 halves of param_2 (r5) / param_3 (r6):
//!   param_3lo > param_2lo            -> (param_2lo << 16) | param_3lo
//!   else flag byte == 0              -> (param_2lo << 16) | param_3lo
//!   else (flag byte != 0)            -> (param_3lo << 16) | param_2lo
//!
//! Hidden side effects the decompiled C does not show (this oracle runs plain
//! bt/bf without their delay slot, so e.g. `mov r7,r0` at 0x8c073678 only runs
//! when the loop is entered — the skipped-loop path leaves r0 = 44):
//! final r0 = param_6*0x30 if the loop ran, else 44; r2 = r15+44 only if the
//! loop ran; r5 := param_3lo and r6/r7 := param_6 only if the loop ran;
//! r8..r14 are restored by the epilogue pops; T ends 0 on every path; the
//! count store at entry+0x28 is the only write outside the stack window.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14,r13,r12,r11,r10,r9,r8,@-r15; add #-12,r15
    for i in [14usize, 13, 12, 11, 10, 9, 8] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[i]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    let r15 = cpu.r[15];

    cpu.r[3] = mem.read_u32(r15.wrapping_add(40)).unwrap(); // param_5 (base)
    cpu.r[8] = cpu.r[5] & 0xFFFF; // extu.w r5,r8
    cpu.r[1] = cpu.r[6] & 0xFFFF; // extu.w r6,r1
    cpu.set_t(cpu.r[1] > cpu.r[8]); // cmp/hi r8,r1
    if cpu.t() {
        // bt 0x8c073662 taken: delay slot 0x8c073656 skipped
        cpu.r[1] = cpu.r[5] & 0xFFFF;
        cpu.r[1] = cpu.r[1] << 16; // shll16 r1
        cpu.r[0] = cpu.r[6] & 0xFFFF;
        cpu.r[1] |= cpu.r[0]; // bra delay slot
    } else {
        cpu.r[0] = mem.read_u32(0x8c073734).unwrap(); // 0x8c1f8210
        cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap() as u32; // mov.b (then extu.b x2)
        cpu.r[0] &= 0xFF;
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            // bf not taken (flag == 0): 0x8c073662 runs as the next instruction
            cpu.r[1] = cpu.r[5] & 0xFFFF;
            cpu.r[1] = cpu.r[1] << 16;
            cpu.r[0] = cpu.r[6] & 0xFFFF;
            cpu.r[1] |= cpu.r[0]; // bra delay slot
        } else {
            // bf 0x8c07366c taken: delay slot skipped, r1 is still extu.w r6
            cpu.r[1] = cpu.r[1] << 16; // shll16 r1
            cpu.r[1] |= cpu.r[8]; // or r8,r1 (r8 = extu.w r5)
        }
    }

    cpu.r[0] = 44;
    cpu.r[8] = mem.read_u32(r15.wrapping_add(44)).unwrap(); // param_6
    cpu.set_t((cpu.r[8] as i32) > (cpu.r[7] as i32)); // cmp/gt r7,r8
    if cpu.t() {
        // loop entered: bf not taken, so 0x8c073678 `mov r7,r0` runs
        let mut r0 = cpu.r[7];
        r0 = r0 << 1; // shll r0 (T set, overwritten before any branch reads it)
        r0 = r0.wrapping_add(cpu.r[7]); // add r7,r0
        r0 = r0 << 2; // shll2 r0
        r0 = r0 << 2; // shll2 r0  => r0 = r7 * 0x30
        cpu.r[9] = cpu.r[3].wrapping_add(2);
        let _ = mem.write_u32(r15.wrapping_add(4), cpu.r[9]);
        cpu.r[14] = cpu.r[5] & 0xFFFF; // extu.w r5,r14
        cpu.r[9] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(r15.wrapping_add(8), cpu.r[9]);
        cpu.r[5] = cpu.r[6] & 0xFFFF; // extu.w r6,r5
        cpu.r[6] = cpu.r[3].wrapping_add(16);
        let _ = mem.write_u32(r15, cpu.r[6]);
        cpu.r[13] = cpu.r[3].wrapping_add(8);
        cpu.r[8] = cpu.r[3].wrapping_add(12);
        cpu.r[12] = cpu.r[3].wrapping_add(40);

        loop {
            // 0x8c0736a4
            cpu.set_t(cpu.r[7] == cpu.r[4]); // cmp/eq r4,r7
            if !cpu.t() {
                cpu.r[6] = mem.read_u32(r0.wrapping_add(cpu.r[12])).unwrap();
                cpu.set_t((cpu.r[6] as i32) > 0); // cmp/pl r6
                if cpu.t() {
                    cpu.r[6] = mem.read_u32(r0.wrapping_add(cpu.r[13])).unwrap();
                    cpu.set_t(cpu.r[6] == cpu.r[1]); // cmp/eq r1,r6
                    if !cpu.t() {
                        cpu.r[6] = mem.read_u32(r0.wrapping_add(cpu.r[8])).unwrap();
                        cpu.set_t(cpu.r[6] == cpu.r[1]);
                    }
                    if !cpu.t() {
                        let p16 = mem.read_u32(r15).unwrap(); // spilled r3+16
                        cpu.r[6] = mem.read_u32(r0.wrapping_add(p16)).unwrap();
                        cpu.set_t(cpu.r[6] == cpu.r[1]);
                    }
                    if cpu.t() {
                        // 0x8c0736c2: reload the flag byte
                        cpu.r[6] = mem.read_u32(0x8c073734).unwrap();
                        cpu.r[6] = mem.read_u8(cpu.r[6]).unwrap() as u32;
                        cpu.r[6] &= 0xFF;
                        cpu.set_t(cpu.r[6] == 0); // tst r6,r6
                        if !cpu.t() {
                            // flag != 0: decrement by 2 (bra delay slot stores)
                            cpu.r[9] = cpu.r[12].wrapping_add(r0);
                            cpu.r[6] = mem.read_u32(cpu.r[9]).unwrap();
                            cpu.r[6] = (cpu.r[6] as i32).wrapping_sub(2) as u32;
                            let _ = mem.write_u32(cpu.r[9], cpu.r[6]);
                        } else {
                            // flag == 0: u16 pair comparisons, decrement by 1 on hit
                            let p2 = mem.read_u32(r15.wrapping_add(4)).unwrap(); // r3+2
                            let p4 = mem.read_u32(r15.wrapping_add(8)).unwrap(); // r3+4
                            cpu.r[10] = mem.read_u16(r0.wrapping_add(p2)).unwrap() as u16 as i16 as i32 as u32;
                            cpu.r[11] = mem.read_u16(r0.wrapping_add(cpu.r[3])).unwrap() as u16 as i16 as i32 as u32;
                            cpu.r[6] = cpu.r[10] & 0xFFFF; // extu.w r10,r6
                            cpu.set_t(cpu.r[6] == cpu.r[14]); // cmp/eq r6,r14
                            // bf.s 0x8c0736f0: delay slot always runs
                            cpu.r[9] = mem.read_u16(r0.wrapping_add(p4)).unwrap() as u16 as i16 as i32 as u32;
                            let mut hit = false;
                            if cpu.t() {
                                cpu.r[6] = cpu.r[11] & 0xFFFF;
                                cpu.set_t(cpu.r[6] == cpu.r[5]); // cmp/eq r6,r5
                                hit = cpu.t();
                            }
                            if !hit {
                                // 0x8c0736f0
                                cpu.r[6] = cpu.r[11] & 0xFFFF;
                                cpu.set_t(cpu.r[6] == cpu.r[14]);
                                if cpu.t() {
                                    cpu.r[6] = cpu.r[9] & 0xFFFF;
                                    cpu.set_t(cpu.r[6] == cpu.r[5]);
                                    hit = cpu.t();
                                }
                            }
                            if !hit {
                                // 0x8c0736fc
                                cpu.r[6] = cpu.r[9] & 0xFFFF;
                                cpu.set_t(cpu.r[6] == cpu.r[14]);
                                if cpu.t() {
                                    cpu.r[6] = cpu.r[10] & 0xFFFF;
                                    cpu.set_t(cpu.r[6] == cpu.r[5]);
                                    hit = cpu.t();
                                }
                            }
                            if hit {
                                // 0x8c073708
                                cpu.r[9] = cpu.r[12].wrapping_add(r0);
                                cpu.r[6] = mem.read_u32(cpu.r[9]).unwrap();
                                cpu.r[6] = (cpu.r[6] as i32).wrapping_sub(1) as u32;
                                let _ = mem.write_u32(cpu.r[9], cpu.r[6]);
                            }
                        }
                    }
                }
            }
            // 0x8c073712
            cpu.r[2] = r15.wrapping_add(44);
            cpu.r[6] = mem.read_u32(cpu.r[2]).unwrap();
            cpu.r[7] = cpu.r[7].wrapping_add(1);
            cpu.set_t((cpu.r[6] as i32) > (cpu.r[7] as i32)); // cmp/gt r7,r6
            r0 = r0.wrapping_add(48); // bt.s delay slot runs either way
            if !cpu.t() {
                break;
            }
        }
        cpu.r[0] = r0;
    }
    // else: loop skipped, bf taken so 0x8c073678 was skipped; r0 stays 44

    // epilogue 0x8c073720: add #12,r15; pop r8..r13; rts (delay pops r14)
    cpu.r[15] = r15.wrapping_add(12);
    for i in [8usize, 9, 10, 11, 12, 13] {
        cpu.r[i] = mem.read_u32(cpu.r[15]).unwrap();
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
