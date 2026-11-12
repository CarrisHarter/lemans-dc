//! 0x8c031e0c — decay a four-slot stick with a 0.13/0.125 friction drag.
//!
//! The object at `r4` holds a normalized value at +0x10, a gate byte at
//! +0x409 and a result byte at +0x411. Four 0xac-byte slots start at +0xb0:
//! a float at +0 and a flag byte at +0x10. When the gate is set and the value
//! is above 0.1, every slot whose float exceeds 1.0 drags the shared value
//! down by 0.13 (0.125 more when the slot's flag byte is set). The value is
//! then clamped to [0, 1] and the result byte is set.
//!
//! `jsr` target 0x8c151b1c (`add #108,r4 ; rts ; r0 = r4`) is the "advance
//! sticks" helper; it is called with `[r4 + 0x24]` and its return is only
//! visible until the first `mova` clobbers r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];

    // mov.l 0x8c031eb8,r0 (0x411) ; mov #0,r1 ; mov.b r1,@(r0,r14)
    cpu.r[0] = mem.read_u32(0x8c031eb8).unwrap_or(0);
    cpu.r[1] = 0;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1] as u8);

    // mov.l 0x8c031ebc,r0 (0x409) ; mov.b @(r0,r14),r0 ; extu.b x2 ; tst r0,r0
    cpu.r[0] = mem.read_u32(0x8c031ebc).unwrap_or(0);
    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[0] &= 0xff;
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // mov r14,r1 ; add #16,r1 ; mova 0x8c031ec0,r0
        cpu.r[1] = cpu.r[14].wrapping_add(16);
        cpu.r[0] = 0x8c031ec0;
        // fmov @r1,fr1 ; fmov @r0,fr0 (0.1) ; fcmp/gt fr0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if cpu.t() {
            // mov.l 0x8c031ec4,r0 (0x8c151b1c) ; jsr @r0 ; delay slot:
            // mov.l @(36,r14),r4
            cpu.r[0] = mem.read_u32(0x8c031ec4).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(36)).unwrap_or(0);
            cpu.pr = 0x8c031e3a;
            crate::registry::p_8c151b1c::run(cpu, mem);

            // mov #0,r1 ; mov r1,r3 ; mov.l 0x8c031ec8,r4 (0xb0)
            // mov r14,r13 ; mov.l 0x8c031ecc,r0 (0xc0) ; mov r14,r7
            // mov r14,r6 ; add r4,r13 ; mov.l 0x8c031ed0,r5 (0xac)
            // add #16,r7 ; add r0,r6 ; mov #4,r4 ; fldi1 fr2
            cpu.r[1] = 0;
            cpu.r[3] = cpu.r[1];
            cpu.r[4] = mem.read_u32(0x8c031ec8).unwrap_or(0);
            cpu.r[13] = cpu.r[14];
            cpu.r[0] = mem.read_u32(0x8c031ecc).unwrap_or(0);
            cpu.r[7] = cpu.r[14];
            cpu.r[6] = cpu.r[14];
            cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[4]);
            cpu.r[5] = mem.read_u32(0x8c031ed0).unwrap_or(0);
            cpu.r[7] = cpu.r[7].wrapping_add(16);
            cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[0]);
            cpu.r[4] = 4;
            cpu.fr[2] = 1.0;

            loop {
                // mov r13,r0 ; add r3,r0 ; fmov @r0,fr0 ; fcmp/gt fr2,fr0
                cpu.r[0] = cpu.r[13].wrapping_add(cpu.r[3]);
                cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
                cpu.set_t(cpu.fr[0] > cpu.fr[2]);
                if cpu.t() {
                    // mova 0x8c031ed4,r0 (0.13) ; fmov @r0,fr0
                    // fmov @r7,fr1 ; mov r3,r0 ; fsub fr0,fr1 ; fmov fr1,@r7
                    cpu.r[0] = 0x8c031ed4;
                    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
                    cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
                    cpu.r[0] = cpu.r[3];
                    cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
                    let _ = mem.write_f32(cpu.r[7], cpu.fr[1]);
                    // mov.b @(r0,r6),r0 ; extu.b x2 ; tst r0,r0
                    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[6])).unwrap_or(0)
                        as i8 as i32 as u32;
                    cpu.r[0] &= 0xff;
                    cpu.r[0] &= 0xff;
                    cpu.set_t(cpu.r[0] == 0);
                    if !cpu.t() {
                        // mova 0x8c031ed8,r0 (0.125) ; fmov @r7,fr1
                        // fsub fr0,fr1 ; fmov fr1,@r7
                        cpu.r[0] = 0x8c031ed8;
                        cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
                        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
                        cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
                        let _ = mem.write_f32(cpu.r[7], cpu.fr[1]);
                    }
                }
                // add #1,r1 ; cmp/gt r1,r4 ; bt.s 0x8c031e54
                // delay slot: add r5,r3
                cpu.r[1] = cpu.r[1].wrapping_add(1);
                let cont = (cpu.r[4] as i32) > (cpu.r[1] as i32);
                cpu.set_t(cont);
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]);
                if !cont {
                    break;
                }
            }

            // mov r14,r0 ; add #16,r0 ; fmov @r0,fr0 ; fldi0 fr1
            // fcmp/gt fr0,fr1 ; bf ... ; else fldi1 fr1 ; fcmp/gt fr1,fr0
            cpu.r[0] = cpu.r[14].wrapping_add(16);
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
            cpu.fr[1] = 0.0;
            cpu.set_t(cpu.fr[1] > cpu.fr[0]);
            if !cpu.t() {
                cpu.fr[1] = 1.0;
                cpu.set_t(cpu.fr[0] > cpu.fr[1]);
                if !cpu.t() {
                    cpu.fr[1] = cpu.fr[0];
                }
            }
            // mov r14,r0 ; add #16,r0 ; fmov fr1,@r0 ; mov #1,r1
            // mov.l 0x8c031eb8,r0 (0x411) ; mov.b r1,@(r0,r14)
            cpu.r[0] = cpu.r[14].wrapping_add(16);
            let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);
            cpu.r[1] = 1;
            cpu.r[0] = mem.read_u32(0x8c031eb8).unwrap_or(0);
            let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1] as u8);
        }
    }

    // lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; delay slot: mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
