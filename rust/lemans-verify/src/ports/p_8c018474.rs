//! 0x8c018474 — spinlock entry: 8 nops, then `tas.b @r4` sets bit 7 of *r4
//! and loops back to the nop block while the byte was nonzero; `rts` (delay
//! nop) once the byte read as zero. Only a zero byte terminates the loop.
//!
//! Side effect the C does not show: the byte at *r4 is ORed with 0x80 on
//! every pass and T ends set (the terminating read was zero). No register
//! changes at all.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        let v = mem.read_u8(cpu.r[4]).unwrap(); // tas.b @r4
        cpu.set_t(v == 0);
        let _ = mem.write_u8(cpu.r[4], v | 0x80);
        if cpu.t() {
            break; // bf 0x8c018474 not taken -> rts ; nop
        }
    }
}
