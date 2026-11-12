//! 0x8c1759ae - flag gate, then store `[r4]` or emit an error string.
//!
//! Twin of 0x8c1759e2 with a plain load instead of the `+8` field. Three exits:
//! `r5 == 0` emits str@0x8c1e0e08 with code 101 and returns 0; `r4 == 0` clears
//! `[r5]` in the `bf.s` delay slot (runs on both paths) and emits
//! str@0x8c1e0e3c, returning 0; otherwise `[r5] = [r4]` and r0 = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        cpu.r[2] = mem.read_u32(0x8c175ad0).unwrap_or(0x8c174564); // mov.l 0x8c175ad0,r2
        cpu.r[5] = mem.read_u32(0x8c175ad8).unwrap_or(0x8c1e0e08); // mov.l 0x8c175ad8,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c1759bc;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c1759d4
    } else {
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        cpu.r[3] = 0; // mov #0,r3
        let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bf.s delay: mov.l r3,@r5
        if cpu.t() {
            cpu.r[3] = mem.read_u32(0x8c175ad0).unwrap_or(0x8c174564); // mov.l 0x8c175ad0,r3
            cpu.r[5] = mem.read_u32(0x8c175adc).unwrap_or(0x8c1e0e3c); // mov.l 0x8c175adc,r5
            cpu.r[4] = 101; // jsr delay: mov #101,r4
            cpu.pr = 0x8c1759d0;
            crate::registry::p_8c174564::run(cpu, mem);
            cpu.r[0] = 0;
        } else {
            cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r2
            cpu.r[0] = 1; // mov #1,r0
            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // mov.l r2,@r5
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
