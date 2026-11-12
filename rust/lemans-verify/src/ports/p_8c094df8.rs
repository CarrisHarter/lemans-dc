//! 0x8c094df8 — one-shot class validator.
//!
//! If the caller's flag byte `[r4+0x83]` is zero, classify the object with
//! 0x8c074b2c(`*[0x8c264da8]`, `[r4+0xf4]`, 0). A class of 4 runs the gates
//! 0x8c074ca0(sub 0 or 1), 0x8c074d58(sub 8), 0x8c074ca0(4), 0x8c074ca0(5);
//! every other class runs 0x8c074d58(sub 8) then 0x8c074ca0(4..7). All gates
//! must pass, after which `[r4+0x83] = 1`. r0 ends 0x83 and r1 = 1 on a stored
//! path, otherwise r0 is the last callee's zero-extended byte result; T is the
//! last `tst`'s.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8C094F0C).unwrap_or(0); // 0x83
    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0) as u32; // mov.b
    cpu.r[0] &= 0xFF; // extu.b
    cpu.r[0] &= 0xFF; // extu.b
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        // 0x8c074b2c(*pool, field, 0)
        cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0); // 0xf4
        cpu.r[6] = 0;
        cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0); // 0x8c264da8
        cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8C094F18).unwrap_or(0); // 0x8c074b2c
        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // delay slot
        crate::registry::p_8c074b2c::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0
        if cpu.r[0] == 4 {
            // ---- class 4 ----
            // 0x8c074ca0(*pool, field, 0)
            cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
            cpu.r[7] = 0;
            cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
            cpu.r[6] = cpu.r[7];
            cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
            cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0); // 0x8c074ca0
            cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
            crate::registry::p_8c074ca0::run(cpu, mem);
            cpu.r[0] &= 0xFF; // extu.b
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            let mut pass = cpu.r[0] != 0; // bf 0x8c094e48
            if !pass {
                // 0x8c074ca0(*pool, field, 1)
                cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                cpu.r[7] = 0;
                cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                cpu.r[6] = 1;
                cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0);
                cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                crate::registry::p_8c074ca0::run(cpu, mem);
                cpu.r[0] &= 0xFF;
                cpu.set_t(cpu.r[0] == 0);
                pass = cpu.r[0] != 0; // bt 0x8c094f06 when zero
            }
            if pass {
                // 0x8c074d58(*pool, field, 8)
                cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                cpu.r[7] = 0;
                cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                cpu.r[6] = 8;
                cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                cpu.r[0] = mem.read_u32(0x8C094F20).unwrap_or(0); // 0x8c074d58
                cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                crate::registry::p_8c074d58::run(cpu, mem);
                cpu.r[0] &= 0xFF;
                cpu.set_t(cpu.r[0] == 0);
                if cpu.r[0] != 0 {
                    // 0x8c074ca0(*pool, field, 4)
                    cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                    cpu.r[7] = 0;
                    cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                    cpu.r[6] = 4;
                    cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                    cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0);
                    cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                    crate::registry::p_8c074ca0::run(cpu, mem);
                    cpu.r[0] &= 0xFF;
                    cpu.set_t(cpu.r[0] == 0);
                    if cpu.r[0] != 0 {
                        // 0x8c074ca0(*pool, field, 5)
                        cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                        cpu.r[7] = 0;
                        cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                        cpu.r[6] = 5;
                        cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                        cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0);
                        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                        crate::registry::p_8c074ca0::run(cpu, mem);
                        cpu.r[0] &= 0xFF;
                        cpu.set_t(cpu.r[0] == 0);
                        if cpu.r[0] != 0 {
                            // mov.l 0x8c094f0c,r0 ; mov #1,r1 ; mov.b r1,@(r0,r14)
                            cpu.r[0] = mem.read_u32(0x8C094F0C).unwrap_or(0);
                            cpu.r[1] = 1;
                            let _ = mem.write_u8(
                                cpu.r[0].wrapping_add(cpu.r[14]),
                                cpu.r[1] as u8,
                            );
                        }
                    }
                }
            }
        } else {
            // ---- class != 4 ----
            // 0x8c074d58(*pool, field, 8)
            cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
            cpu.r[7] = 0;
            cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
            cpu.r[6] = 8;
            cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
            cpu.r[0] = mem.read_u32(0x8C094F20).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
            crate::registry::p_8c074d58::run(cpu, mem);
            cpu.r[0] &= 0xFF;
            cpu.set_t(cpu.r[0] == 0);
            if cpu.r[0] != 0 {
                // 0x8c074ca0(*pool, field, 4)
                cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                cpu.r[7] = 0;
                cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                cpu.r[6] = 4;
                cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0);
                cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                crate::registry::p_8c074ca0::run(cpu, mem);
                cpu.r[0] &= 0xFF;
                cpu.set_t(cpu.r[0] == 0);
                if cpu.r[0] != 0 {
                    // 0x8c074ca0(*pool, field, 5)
                    cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                    cpu.r[7] = 0;
                    cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                    cpu.r[6] = 5;
                    cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                    cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0);
                    cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                    crate::registry::p_8c074ca0::run(cpu, mem);
                    cpu.r[0] &= 0xFF;
                    cpu.set_t(cpu.r[0] == 0);
                    if cpu.r[0] != 0 {
                        // 0x8c074ca0(*pool, field, 6)
                        cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                        cpu.r[7] = 0;
                        cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                        cpu.r[6] = 6;
                        cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                        cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0);
                        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                        crate::registry::p_8c074ca0::run(cpu, mem);
                        cpu.r[0] &= 0xFF;
                        cpu.set_t(cpu.r[0] == 0);
                        if cpu.r[0] != 0 {
                            // 0x8c074ca0(*pool, field, 7)
                            cpu.r[0] = mem.read_u32(0x8C094F10).unwrap_or(0);
                            cpu.r[7] = 0;
                            cpu.r[1] = mem.read_u32(0x8C094F14).unwrap_or(0);
                            cpu.r[6] = 7;
                            cpu.r[5] =
                                mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
                            cpu.r[0] = mem.read_u32(0x8C094F1C).unwrap_or(0);
                            cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                            crate::registry::p_8c074ca0::run(cpu, mem);
                            cpu.r[0] &= 0xFF;
                            cpu.set_t(cpu.r[0] == 0);
                            if cpu.r[0] != 0 {
                                // fell through to the shared store
                                cpu.r[0] = mem.read_u32(0x8C094F0C).unwrap_or(0);
                                cpu.r[1] = 1;
                                let _ = mem.write_u8(
                                    cpu.r[0].wrapping_add(cpu.r[14]),
                                    cpu.r[1] as u8,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // lds.l @r15+,pr ; rts ; mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
