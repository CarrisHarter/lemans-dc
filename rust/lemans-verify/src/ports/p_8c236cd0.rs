//! 0x8c236cd0 - unlink the node r5 from the list headed at r4.
//!
//! Nodes link through `+4`. If r5 is the head, `[r4] = [r5+4]`. Otherwise the
//! predecessor (the node whose link equals r5) gets that link. The walk stops
//! when it finds that predecessor or the link chain ends.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[7] = 0;
    cpu.set_t(cpu.r[6] == 0); // tst r6,r6
    cpu.r[0] = cpu.r[7]; // bt.s delay
    if !cpu.t() {
        loop {
            cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
            cpu.set_t(cpu.r[5] == cpu.r[2]); // cmp/eq r5,r2
            if cpu.t() {
                cpu.r[0] = cpu.r[6];
                cpu.r[6] = cpu.r[7]; // bra delay
                break;
            }
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
            cpu.set_t(cpu.r[6] == 0);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.set_t(cpu.r[5] == cpu.r[2]); // cmp/eq r5,r2
    if cpu.t() {
        cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[4], cpu.r[1]); // bra delay
    } else {
        cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[2]);
    }
}
