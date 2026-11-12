//! 0x8c091b34 — reduce `param_2` rows of stride 0x148 into a 0x148-byte
//! record of per-column maxima, then copy that record to the pointer passed
//! on the stack at entry (`[SP]`, the third argument).
//!
//! The 46-column block `[0x00,0xb8)` and the 36-column block `[0xb8,0x148)`
//! are handled separately. Each column starts at 0 and takes the signed
//! maximum of `param_1 + col*4 + row*0x148` over `row < param_2`; a
//! non-positive `param_2` leaves the record zeroed. The local record is the
//! 0x148-byte buffer at the frame base, zeroed by the called 0x8c091a80.
//!
//! Side effects beyond the 0x148-byte copy: r0 is the last word copied
//! (column 0x35 of the second block), r1 = 0, r3 = frame+0x148,
//! r4 = frame+0xb8, r5 = param_2 when it is positive (else 0), r6 = 0x90 and
//! r7 = 0x148. The final `dt` leaves T set. r2, r8, r9-r14, pr and r15 keep
//! their entry values; the epilogue pops r9-r13 and the `rts` delay slot pops
//! r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Frame pushes, then 0x164 bytes of locals (0x148 record + saved regs).
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(128);
    cpu.r[15] = cpu.r[15].wrapping_sub(128);
    cpu.r[0] = mem.read_u32(0x8c091c24).unwrap_or(0); // 0x164
    cpu.r[15] = cpu.r[15].wrapping_sub(72);
    cpu.r[12] = mem
        .read_u32(cpu.r[15].wrapping_add(cpu.r[0]))
        .unwrap_or(0); // stack-passed destination pointer

    cpu.r[0] = mem.read_u32(0x8c091c28).unwrap_or(0); // 0x8c091a80
    cpu.pr = 0x8c091b56; // jsr return address
    cpu.r[4] = cpu.r[15]; // delay slot
    crate::registry::p_8c091a80::run(cpu, mem);

    // 0x8c091b56: first (46-column) block of maxima.
    cpu.r[11] = mem.read_u32(0x8c091c2c).unwrap_or(0); // 0x148
    cpu.r[4] = 0;
    cpu.r[7] = cpu.r[4];
    cpu.r[1] = cpu.r[4];
    cpu.r[3] = cpu.r[15];
    cpu.r[9] = 46;
    loop {
        cpu.r[0] = cpu.r[3];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[7]), cpu.r[1]);
        cpu.set_t((cpu.r[14] as i32) > 0); // cmp/pl
        cpu.r[5] = cpu.r[1]; // bf.s delay slot, both paths
        if cpu.t() {
            cpu.r[6] = cpu.r[1];
            loop {
                cpu.r[10] = cpu.r[13];
                cpu.r[0] = cpu.r[7];
                cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[6]);
                cpu.r[10] = mem
                    .read_u32(cpu.r[10].wrapping_add(cpu.r[0]))
                    .unwrap_or(0);
                cpu.r[0] = cpu.r[3];
                cpu.r[0] = mem
                    .read_u32(cpu.r[0].wrapping_add(cpu.r[7]))
                    .unwrap_or(0);
                cpu.set_t((cpu.r[10] as i32) > (cpu.r[0] as i32)); // cmp/gt
                if !cpu.t() {
                    cpu.r[10] = cpu.r[0]; // bf path; bra path keeps the new max
                }
                cpu.r[0] = cpu.r[3];
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[7]), cpu.r[10]);
                cpu.set_t((cpu.r[14] as i32) > (cpu.r[5] as i32)); // cmp/gt
                cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[11]); // bt.s delay slot
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t((cpu.r[9] as i32) > (cpu.r[4] as i32)); // cmp/gt
        cpu.r[7] = cpu.r[7].wrapping_add(4); // bt.s delay slot
        if !cpu.t() {
            break;
        }
    }

    // 0x8c091b98: second (36-column) block, starting at record + 0xb8.
    cpu.r[7] = mem.read_u32(0x8c091c2c).unwrap_or(0); // 0x148
    cpu.r[11] = 0;
    cpu.r[6] = cpu.r[11];
    cpu.r[0] = mem.read_u32(0x8c091c30).unwrap_or(0); // 0xb8
    cpu.r[1] = cpu.r[11];
    cpu.r[3] = mem.read_u32(0x8c091c30).unwrap_or(0); // 0xb8
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]);
    cpu.r[9] = 36;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[15]);
    loop {
        cpu.r[0] = cpu.r[3];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[6]), cpu.r[1]);
        cpu.set_t((cpu.r[14] as i32) > 0); // cmp/pl
        cpu.r[4] = cpu.r[1]; // bf.s delay slot, both paths
        if cpu.t() {
            cpu.r[5] = cpu.r[1];
            loop {
                cpu.r[10] = cpu.r[13];
                cpu.r[0] = cpu.r[6];
                cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[5]);
                cpu.r[10] = mem
                    .read_u32(cpu.r[10].wrapping_add(cpu.r[0]))
                    .unwrap_or(0);
                cpu.r[0] = cpu.r[3];
                cpu.r[0] = mem
                    .read_u32(cpu.r[0].wrapping_add(cpu.r[6]))
                    .unwrap_or(0);
                cpu.set_t((cpu.r[10] as i32) > (cpu.r[0] as i32)); // cmp/gt
                if !cpu.t() {
                    cpu.r[10] = cpu.r[0];
                }
                cpu.r[0] = cpu.r[3];
                cpu.r[4] = cpu.r[4].wrapping_add(1);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[6]), cpu.r[10]);
                cpu.set_t((cpu.r[14] as i32) > (cpu.r[4] as i32)); // cmp/gt
                cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[7]); // bt.s delay slot
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[11] = cpu.r[11].wrapping_add(1);
        cpu.set_t((cpu.r[9] as i32) > (cpu.r[11] as i32)); // cmp/gt
        cpu.r[6] = cpu.r[6].wrapping_add(4); // bt.s delay slot
        if !cpu.t() {
            break;
        }
    }

    // 0x8c091be0: copy the first 0xb8 bytes to the destination.
    cpu.r[3] = cpu.r[12];
    cpu.r[4] = cpu.r[15];
    cpu.r[1] = 23;
    loop {
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        cpu.r[1] = cpu.r[1].wrapping_sub(1);
        cpu.set_t(cpu.r[1] == 0); // dt: T set when the counter reaches zero
        let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
        cpu.r[3] = cpu.r[3].wrapping_add(8); // bf.s delay slot
        if cpu.t() {
            break;
        }
    }

    // 0x8c091bf4: copy the remaining 0x90 bytes.
    cpu.r[0] = mem.read_u32(0x8c091c30).unwrap_or(0); // 0xb8
    cpu.r[1] = 18;
    cpu.r[3] = mem.read_u32(0x8c091c30).unwrap_or(0); // 0xb8
    cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[15]);
    loop {
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_add(4);
        cpu.r[1] = cpu.r[1].wrapping_sub(1);
        cpu.set_t(cpu.r[1] == 0); // dt: T set when the counter reaches zero
        let _ = mem.write_u32(cpu.r[12], cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[12].wrapping_add(4), cpu.r[0]);
        cpu.r[12] = cpu.r[12].wrapping_add(8); // bf.s delay slot
        if cpu.t() {
            break;
        }
    }

    // 0x8c091c0c: drop the frame, restore saved registers and pr.
    cpu.r[15] = cpu.r[15].wrapping_add(124);
    cpu.r[15] = cpu.r[15].wrapping_add(124);
    cpu.r[15] = cpu.r[15].wrapping_add(80);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
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
