//! 0x8c0209a0 — compare two descriptor records (three sub-records each).
//!
//! Returns 1 when the records match:
//! - field 3 matches after masking with 0x1de6e1, unless either side carries
//!   bit 0x100 (then the field is ignored);
//! - field 4 is equal, or either side is the 8 sentinel;
//! - field 5 is equal, or either side is the 9 sentinel;
//! - the first word of each of the three referenced sub-records is equal;
//! - the strings at each sub-record's offset 12 compare equal via the
//!   case-insensitive compare at 0x8c070ac8 (bound = the sub-record's word 0).
//!
//! Side effects: pushes r14/r13/pr and pops them back; r0 = 0/1, r1 = second
//! record's third pointer, r3 = first record's third pointer, r4/r5/r6 as left
//! by the last callee call, T = result.

use sh4_oracle::{Cpu, Memory};

const CMP: u32 = 0x8c070ac8;

fn ld(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0209a0..0x8c0209a8 — prologue
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];

    let result;
    'done: {
        // 0x8c0209aa — r4 = a[3]; r3 = 0x100; tst r0,r0 after `and`
        cpu.r[4] = ld(mem, cpu.r[13].wrapping_add(12));
        cpu.r[3] = 0x100;
        cpu.r[0] = cpu.r[4] & cpu.r[3];
        cpu.set_t(cpu.r[0] == 0);
        let mut proceed = cpu.r[0] != 0; // bf 0x8c0209ce
        if !proceed {
            // 0x8c0209b6 — r1 = b[3]; tst r0,r0 after `and`
            cpu.r[1] = ld(mem, cpu.r[14].wrapping_add(12));
            cpu.r[0] = cpu.r[1] & cpu.r[3];
            cpu.set_t(cpu.r[0] == 0);
            proceed = cpu.r[0] != 0; // bf 0x8c0209ce
            if !proceed {
                // 0x8c0209c0 — mask both and compare
                cpu.r[0] = 0x001d_e6e1;
                cpu.r[4] &= cpu.r[0];
                cpu.r[1] &= cpu.r[0];
                cpu.set_t(cpu.r[4] == cpu.r[1]); // cmp/eq r1,r4
                proceed = cpu.r[4] == cpu.r[1]; // bt 0x8c0209ce
            }
        }
        if !proceed {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c0209ce — field 4: equal or either side is 8
        cpu.r[1] = ld(mem, cpu.r[13].wrapping_add(16));
        cpu.r[0] = cpu.r[1];
        cpu.set_t(cpu.r[0] == 8);
        let mut proceed = cpu.r[0] == 8; // bt 0x8c0209e4
        if !proceed {
            cpu.r[0] = ld(mem, cpu.r[14].wrapping_add(16));
            cpu.set_t(cpu.r[0] == 8);
            proceed = cpu.r[0] == 8; // bt 0x8c0209e4
            if !proceed {
                cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r0,r1
                proceed = cpu.r[1] == cpu.r[0]; // bt 0x8c0209e4
            }
        }
        if !proceed {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c0209e4 — field 5: equal or either side is 9
        cpu.r[1] = ld(mem, cpu.r[13].wrapping_add(20));
        cpu.r[0] = cpu.r[1];
        cpu.set_t(cpu.r[0] == 9);
        let mut proceed = cpu.r[0] == 9; // bt 0x8c0209fa
        if !proceed {
            cpu.r[0] = ld(mem, cpu.r[14].wrapping_add(20));
            cpu.set_t(cpu.r[0] == 9);
            proceed = cpu.r[0] == 9; // bt 0x8c0209fa
            if !proceed {
                cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r0,r1
                proceed = cpu.r[1] == cpu.r[0]; // bt 0x8c0209fa
            }
        }
        if !proceed {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c0209fa — *a[0] == *b[0]
        cpu.r[1] = ld(mem, cpu.r[13]);
        cpu.r[0] = ld(mem, cpu.r[14]);
        cpu.r[6] = ld(mem, cpu.r[1]);
        cpu.r[3] = ld(mem, cpu.r[0]);
        cpu.set_t(cpu.r[6] == cpu.r[3]); // cmp/eq r3,r6
        if cpu.r[6] != cpu.r[3] {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c020a0a — *a[1] == *b[1]
        cpu.r[4] = ld(mem, cpu.r[13].wrapping_add(4));
        cpu.r[3] = ld(mem, cpu.r[14].wrapping_add(4));
        cpu.r[4] = ld(mem, cpu.r[4]);
        cpu.r[3] = ld(mem, cpu.r[3]);
        cpu.set_t(cpu.r[4] == cpu.r[3]); // cmp/eq r3,r4
        if cpu.r[4] != cpu.r[3] {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c020a1a — *a[2] == *b[2]
        cpu.r[4] = ld(mem, cpu.r[13].wrapping_add(8));
        cpu.r[3] = ld(mem, cpu.r[14].wrapping_add(8));
        cpu.r[4] = ld(mem, cpu.r[4]);
        cpu.r[3] = ld(mem, cpu.r[3]);
        cpu.set_t(cpu.r[4] == cpu.r[3]); // cmp/eq r3,r4
        if cpu.r[4] != cpu.r[3] {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c020a2a — compare the first sub-record's strings
        cpu.r[5] = ld(mem, cpu.r[0].wrapping_add(12)); // b[0][3]
        cpu.r[0] = CMP;
        cpu.pr = 0x8c020a32; // jsr return address
        cpu.r[4] = ld(mem, cpu.r[1].wrapping_add(12)); // a[0][3] (delay slot)
        crate::registry::p_8c070ac8::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c020a3a — compare the second sub-record's strings
        cpu.r[3] = ld(mem, cpu.r[13].wrapping_add(4)); // a[1]
        cpu.r[1] = ld(mem, cpu.r[14].wrapping_add(4)); // b[1]
        cpu.r[0] = CMP;
        cpu.pr = 0x8c020a48; // jsr return address
        cpu.r[6] = ld(mem, cpu.r[3]); // *a[1]
        cpu.r[5] = ld(mem, cpu.r[1].wrapping_add(12)); // b[1][3]
        cpu.r[4] = ld(mem, cpu.r[3].wrapping_add(12)); // a[1][3] (delay slot)
        crate::registry::p_8c070ac8::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            cpu.r[0] = 0; // bra delay slot
            result = 0;
            break 'done;
        }

        // 0x8c020a50 — compare the third sub-record's strings
        cpu.r[3] = ld(mem, cpu.r[13].wrapping_add(8)); // a[2]
        cpu.r[1] = ld(mem, cpu.r[14].wrapping_add(8)); // b[2]
        cpu.r[0] = CMP;
        cpu.pr = 0x8c020a5e; // jsr return address
        cpu.r[6] = ld(mem, cpu.r[3]); // *a[2]
        cpu.r[5] = ld(mem, cpu.r[1].wrapping_add(12)); // b[2][3]
        cpu.r[4] = ld(mem, cpu.r[3].wrapping_add(12)); // a[2][3] (delay slot)
        crate::registry::p_8c070ac8::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        result = u32::from(cpu.r[0] == 0); // movt r0
    }

    // 0x8c020a62 — epilogue
    cpu.r[0] = result;
    cpu.pr = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = ld(mem, cpu.r[15]); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
