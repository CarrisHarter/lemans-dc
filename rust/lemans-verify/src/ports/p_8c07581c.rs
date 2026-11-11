//! 0x8c07581c — clear a run of records.
//!
//! ```text
//! r1 = [r4+56] ; r0 = 72 ; r3 = 0 ; [r4+72] = r1
//! r5 = [r4+60] ; r6 = [r4+56] ; bra tail ; r4 += 68
//! loop: r1 = [r6] ; [r1+20]=0 ; r6+=4 ; [r1+28]=0 ; [r1+36]=0
//!       [r1+44]=0 ; [r1+48]=0 ; [r1+52]=0 ; [r1+72]=0 ; [r5]=0 ; r5+=1
//! tail: r1 = [r4] ; cmp/hi r6,r1 ; bt loop
//! rts
//! ```
//!
//! Hidden side effects: `r4` is advanced by 68 before the loop (so the bound is
//! read from `orig+68`), `r5` walks one byte per record, `r6` ends at the bound,
//! and `r0`/`r3` keep 72 and 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let start = mem.read_u32(base + 56).unwrap_or(0);
    let byte_base = mem.read_u32(base + 60).unwrap_or(0);
    let _ = mem.write_u32(base + 72, start);

    cpu.r[0] = 72;
    cpu.r[3] = 0;
    cpu.r[5] = byte_base;
    cpu.r[6] = start;
    cpu.r[4] = base.wrapping_add(68); // delay slot of `bra`

    loop {
        let bound = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[1] = bound;
        if bound <= cpu.r[6] {
            break; // cmp/hi r6,r1 / bt
        }
        let rec = mem.read_u32(cpu.r[6]).unwrap_or(0);
        cpu.r[1] = rec;
        for off in [20u32, 28, 36, 44, 48, 52] {
            let _ = mem.write_u32(rec + off, 0);
        }
        cpu.r[6] = cpu.r[6].wrapping_add(4);
        let _ = mem.write_u8(rec + 72, 0);
        let _ = mem.write_u8(cpu.r[5], 0);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
    }
    cpu.set_t(false); // cmp/hi was false at exit
}
