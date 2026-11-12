//! 0x8c08f5fc — normalise a counted array of 44-byte float nodes.
//!
//! Called with the array descriptor in `r4`: node base at `[r4+56]`, node
//! count at `[r4+60]`, processed counter at `[r4+64]` and a done flag byte at
//! `[r4+68]`. For each node whose float at offset 12 is `> 0.001f` (the
//! literal at 0x8c08f6b8) and whose byte at offset 40 is zero, it performs
//! four in-place three-float adds through 0x8c0b4f34 (`a += b`, `a += c`,
//! `b += c`, `b += c`), subtracts the constant float at 0x8c1d1cd0 from the
//! node's offset-12 float, and bumps the processed counter. If the loop ends
//! without a single processed node, the done flag is set to 1.
//!
//! The loop bound is computed exactly as the original: `macl` of
//! `mul.l 44,count`, then an unsigned `cmp/hi` against the base. The callee
//! port is invoked at all four `jsr` sites with the delay-slot argument
//! registers already applied.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[12] = cpu.r[4];
    cpu.r[0] = 64;
    cpu.r[3] = 0;
    cpu.r[1] = cpu.r[12];
    let _ = mem.write_u32(cpu.r[12].wrapping_add(cpu.r[0]), cpu.r[3]); // [r12+64] = 0
    cpu.r[1] = cpu.r[1].wrapping_add(68);
    cpu.r[0] = mem.read_u8(cpu.r[1]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r1,r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0);
    if !cpu.t() {
        // bf 0x8c08f6ae: done flag already set, straight to the epilogue.
        epilogue(cpu, mem);
        return;
    }

    cpu.r[1] = mem.read_u32(cpu.r[12].wrapping_add(60)).unwrap_or(0); // count
    cpu.r[0] = 44;
    cpu.r[14] = mem.read_u32(cpu.r[12].wrapping_add(56)).unwrap_or(0); // node base
    // mul.l r0,r1 ; sts macl,r1
    let product = (cpu.r[1] as i32 as i64) * (cpu.r[0] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[1] = cpu.macl;
    cpu.r[13] = cpu.r[14].wrapping_add(cpu.r[1]);
    cpu.set_t(cpu.r[13] > cpu.r[14]); // cmp/hi r14,r13 (unsigned)
    if cpu.t() {
        loop {
            cpu.r[1] = cpu.r[14].wrapping_add(12);
            cpu.r[0] = 0x8c08f6b8; // mova @(disp,pc),r0
            cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0.001f
            cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
            let above = cpu.t();
            cpu.r[1] = 0; // bf.s delay slot: mov #0,r1
            if above {
                cpu.r[0] = cpu.r[14].wrapping_add(40);
                cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
                cpu.r[0] &= 0xff; // extu.b r0,r0
                cpu.r[0] &= 0xff; // extu.b r0,r0
                cpu.set_t(cpu.r[0] & cpu.r[0] == 0);
                if cpu.t() {
                    cpu.r[1] = 1;
                }
            }
            cpu.r[0] = cpu.r[1] & 0xff; // extu.b r1,r0
            cpu.set_t(cpu.r[0] & cpu.r[0] == 0);
            if !cpu.t() {
                // a += b
                cpu.r[0] = mem.read_u32(0x8c08f6bc).unwrap_or(0);
                cpu.r[5] = cpu.r[14];
                cpu.r[4] = cpu.r[14];
                cpu.pr = 0x8c08f65c; // jsr @r0
                cpu.r[5] = cpu.r[14].wrapping_add(16); // delay slot
                crate::registry::p_8c0b4f34::run(cpu, mem);

                // a += c
                cpu.r[0] = mem.read_u32(0x8c08f6bc).unwrap_or(0);
                cpu.r[5] = cpu.r[14];
                cpu.r[4] = cpu.r[14];
                cpu.pr = 0x8c08f666;
                cpu.r[5] = cpu.r[14].wrapping_add(28); // delay slot
                crate::registry::p_8c0b4f34::run(cpu, mem);

                // b += c
                cpu.r[0] = mem.read_u32(0x8c08f6bc).unwrap_or(0);
                cpu.r[5] = cpu.r[14];
                cpu.r[4] = cpu.r[14];
                cpu.r[5] = cpu.r[14].wrapping_add(28);
                cpu.pr = 0x8c08f672;
                cpu.r[4] = cpu.r[14].wrapping_add(16); // delay slot
                crate::registry::p_8c0b4f34::run(cpu, mem);

                // b += c (the original really repeats the third call)
                cpu.r[0] = mem.read_u32(0x8c08f6bc).unwrap_or(0);
                cpu.r[5] = cpu.r[14];
                cpu.r[4] = cpu.r[14];
                cpu.r[5] = cpu.r[14].wrapping_add(28);
                cpu.pr = 0x8c08f67e;
                cpu.r[4] = cpu.r[14].wrapping_add(16); // delay slot
                crate::registry::p_8c0b4f34::run(cpu, mem);

                cpu.r[0] = mem.read_u32(0x8c08f6c0).unwrap_or(0); // 0x8c1d1cd0
                cpu.r[3] = cpu.r[14].wrapping_add(12);
                cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
                cpu.r[1] = cpu.r[12];
                cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
                cpu.r[0] = 64;
                cpu.r[1] = cpu.r[1].wrapping_add(64);
                cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
                let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
                cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                cpu.r[1] = cpu.r[1].wrapping_add(1);
                let _ = mem.write_u32(cpu.r[12].wrapping_add(cpu.r[0]), cpu.r[1]);
            }
            cpu.r[14] = cpu.r[14].wrapping_add(44);
            cpu.set_t(cpu.r[13] > cpu.r[14]); // cmp/hi r14,r13
            if !cpu.t() {
                break;
            }
        }
    }

    cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(64)).unwrap_or(0);
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = 68;
        cpu.r[1] = 1;
        let _ = mem.write_u8(cpu.r[12].wrapping_add(cpu.r[0]), cpu.r[1] as u8);
    }

    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // lds.l @r15+,pr ; mov.l @r15+,r12 ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
