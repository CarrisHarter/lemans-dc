//! 0x8c16eaec — if ([r4+8] & 1) != 0, copy fr4/fr5/fr6 to [r4+92]+60/64/68,
//! then copy those three words to +72/76/80 and OR 0x01800000 (pool word at
//! 0x8c16eb2c) into [r4+8]. Leaked: r0=0x01800000, r1=[r4+8]|0x01800000,
//! r3=[obj+60], r5=obj+72, r6=obj+60, T=0. Bit clear: r0=0, T=1, no stores.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r0 = mem.read_u32(r4 + 8).unwrap() & 1;
    cpu.set_t(r0 == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = r0;
        return; // rts ; nop
    }

    let obj = mem.read_u32(r4 + 92).unwrap();
    let flag = mem.read_u32(0x8c16eb2c).unwrap(); // 0x01800000
    mem.write_f32(obj + 60, cpu.fr[4]).unwrap();
    mem.write_f32(obj + 64, cpu.fr[5]).unwrap();
    mem.write_f32(obj + 68, cpu.fr[6]).unwrap();
    let w0 = mem.read_u32(obj + 60).unwrap();
    let w1 = mem.read_u32(obj + 64).unwrap();
    let w2 = mem.read_u32(obj + 68).unwrap();
    mem.write_u32(obj + 72, w0).unwrap();
    mem.write_u32(obj + 76, w1).unwrap();
    mem.write_u32(obj + 80, w2).unwrap();
    let r1 = mem.read_u32(r4 + 8).unwrap() | flag;
    mem.write_u32(r4 + 8, r1).unwrap();

    cpu.r[0] = flag;
    cpu.r[1] = r1;
    cpu.r[3] = w0;
    cpu.r[5] = obj + 72;
    cpu.r[6] = obj + 60;
}
