//! 0x8c074264 — swap two 0x30-byte records and remap their reference slots.
//!
//! r4/r5 are the two record indices, r6 the record array base, r7 the number of
//! records to scan. `cmp/eq r10,r9` makes an equal-index call a no-op (T set).
//! Otherwise the two 48-byte records are swapped through a 48-byte stack buffer
//! by three memmove calls (0x8c010e84, the literal at 0x8c074304). Record
//! offsets are built as `idx*3 << 4` (`shll`, `add`, `shll2`, `shll2`).
//! With r7 <= 0 the routine stops after the swap; otherwise it walks each
//! record's five 32-bit slots at +0x14..+0x27, breaking the inner scan on the
//! first negative slot, and rewrites references equal to r4 as r5 and r5 as r4.
//! The `bf.s` at 0x8c0742b8 and both `bt.s` loops run their delay slot on every
//! pass, taken or not.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: push r14, r13, r12, r11, r10, r9 then pr; 48-byte local buffer
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[7]; // mov r7,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[11] = cpu.r[6]; // mov r6,r11
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[10] = cpu.r[5]; // mov r5,r10
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(48); // add #-48,r15
    cpu.r[9] = cpu.r[4]; // mov r4,r9
    cpu.set_t(cpu.r[9] == cpu.r[10]); // cmp/eq r10,r9
    if cpu.r[9] == cpu.r[10] {
        return epilogue(cpu, mem); // bt 0x8c0742f0
    }

    // r13 = r9*0x30 + r11
    cpu.r[13] = cpu.r[9] << 1; // shll
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[9]); // add r9,r13
    cpu.r[13] <<= 2; // shll2
    cpu.r[13] <<= 2; // shll2
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[11]); // add r11,r13
    cpu.r[0] = mem.read_u32(0x8c074304).unwrap_or(0x8c010e84); // mov.l <memmove>,r0
    cpu.r[5] = cpu.r[13];
    cpu.r[4] = cpu.r[15];
    cpu.pr = 0x8c074296;
    cpu.r[6] = 48; // jsr delay slot
    crate::registry::p_8c010e84::run(cpu, mem);

    // r14 = r10*0x30 + r11
    cpu.r[14] = cpu.r[10] << 1; // shll
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[10]); // add r10,r14
    cpu.r[14] <<= 2; // shll2
    cpu.r[14] <<= 2; // shll2
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[11]); // add r11,r14
    cpu.r[0] = mem.read_u32(0x8c074304).unwrap_or(0x8c010e84); // mov.l <memmove>,r0
    cpu.r[5] = cpu.r[14];
    cpu.r[4] = cpu.r[13];
    cpu.pr = 0x8c0742aa;
    cpu.r[6] = 48; // jsr delay slot
    crate::registry::p_8c010e84::run(cpu, mem);

    cpu.r[0] = mem.read_u32(0x8c074304).unwrap_or(0x8c010e84); // mov.l <memmove>,r0
    cpu.r[4] = cpu.r[14];
    cpu.r[5] = cpu.r[15];
    cpu.pr = 0x8c0742b4;
    cpu.r[6] = 48; // jsr delay slot
    crate::registry::p_8c010e84::run(cpu, mem);

    cpu.set_t((cpu.r[12] as i32) > 0); // cmp/pl r12
    cpu.r[6] = 0; // bf.s delay slot (runs whether or not we branch)
    if (cpu.r[12] as i32) > 0 {
        cpu.r[13] = 0;
        cpu.r[5] = 0;
        cpu.r[11] = cpu.r[11].wrapping_add(20); // add #20,r11
        cpu.r[1] = 5;
        loop {
            cpu.r[7] = cpu.r[5];
            cpu.r[0] = cpu.r[5];
            cpu.r[4] = cpu.r[11].wrapping_add(cpu.r[13]);
            loop {
                let at = cpu.r[4].wrapping_add(cpu.r[0]);
                cpu.r[3] = mem.read_u32(at).unwrap_or(0); // mov.l @(r0,r4),r3
                cpu.set_t((cpu.r[3] as i32) >= 0); // cmp/pz r3
                if (cpu.r[3] as i32) < 0 {
                    break; // bf 0x8c0742e8
                }
                cpu.set_t(cpu.r[3] == cpu.r[9]); // cmp/eq r3,r9
                if cpu.r[3] == cpu.r[9] {
                    let _ = mem.write_u32(at, cpu.r[10]); // bra delay slot: mov.l r10,@(r0,r4)
                } else {
                    cpu.set_t(cpu.r[3] == cpu.r[10]); // cmp/eq r3,r10
                    if cpu.r[3] == cpu.r[10] {
                        let _ = mem.write_u32(at, cpu.r[9]); // mov.l r9,@(r0,r4)
                    }
                }
                cpu.r[7] = cpu.r[7].wrapping_add(1);
                cpu.set_t((cpu.r[1] as i32) > (cpu.r[7] as i32)); // cmp/gt r7,r1
                let more = cpu.t();
                cpu.r[0] = cpu.r[0].wrapping_add(4); // bt.s delay slot
                if !more {
                    break;
                }
            }
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.set_t((cpu.r[12] as i32) > (cpu.r[6] as i32)); // cmp/gt r6,r12
            let more = cpu.t();
            cpu.r[13] = cpu.r[13].wrapping_add(48); // bt.s delay slot
            if !more {
                break;
            }
        }
    }
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(48);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
