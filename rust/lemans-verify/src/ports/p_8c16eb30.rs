//! 0x8c16eb30 — like 0x8c16eaec, but the three source words come from the
//! stack at [r15]/[r15+4]/[r15+8] instead of fr4..fr6.
//!
//! If ([r4+8] & 1) != 0: [obj+60/64/68] = [r15]/[r15+4]/[r15+8] where
//! obj=[r4+92], then those words copied to +72/76/80, and 0x01800000 (pool
//! at 0x8c16eb70) ORed into [r4+8]. Leaked: r0=0x01800000,
//! r1=[r4+8]|0x01800000, r3=[obj+60], r5=obj+72, r6=obj+60, r7=r4+92, T=0.
//! Bit clear: r0=0, T=1, no stores.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r0 = mem.read_u32(r4 + 8).unwrap() & 1;
    cpu.set_t(r0 == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = r0;
        return; // rts ; nop
    }

    let r7 = r4 + 92;
    let obj = mem.read_u32(r7).unwrap();
    let flag = mem.read_u32(0x8c16eb70).unwrap(); // 0x01800000
    let r15 = cpu.r[15];
    let w0 = mem.read_u32(r15).unwrap();
    let w1 = mem.read_u32(r15 + 4).unwrap();
    let w2 = mem.read_u32(r15 + 8).unwrap();
    mem.write_u32(obj + 60, w0).unwrap();
    mem.write_u32(obj + 64, w1).unwrap();
    mem.write_u32(obj + 68, w2).unwrap();
    let c0 = mem.read_u32(obj + 60).unwrap();
    let c1 = mem.read_u32(obj + 64).unwrap();
    let c2 = mem.read_u32(obj + 68).unwrap();
    mem.write_u32(obj + 72, c0).unwrap();
    mem.write_u32(obj + 76, c1).unwrap();
    mem.write_u32(obj + 80, c2).unwrap();
    let r1 = mem.read_u32(r4 + 8).unwrap() | flag;
    mem.write_u32(r4 + 8, r1).unwrap();

    cpu.r[0] = flag;
    cpu.r[1] = r1;
    cpu.r[3] = c0;
    cpu.r[5] = obj + 72;
    cpu.r[6] = obj + 60;
    cpu.r[7] = r7;
}
