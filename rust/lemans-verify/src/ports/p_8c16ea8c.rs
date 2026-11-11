//! 0x8c16ea8c — flag-gated update of a sub-object.
//!
//! If ([r4+8] & 1) == 0: return immediately with r0=0, T=1, no stores.
//! Otherwise p = [r4+92] + 40; if f32[p] != fr4 store fr4 at p (fcmp/eq).
//! Then on extu.b(r5) != 0: fr0 = f32[pool 0x8c1ffe10], fr1 = f32[p] / fr0
//! (note: read after the possible fr4 store), store fr1 at [r4+92]+44, and
//! the rts delay slot writes byte 1 at [r4+92]+48; r4 is left at r4+92,
//! r0=48, r1=1, r3=[r4+92], T=0. On extu.b(r5) == 0: delay-slot-free store
//! of byte 0 at [r4+92]+48, r0=48, r1=0, T=1, fr0 keeps the f32[p] load.

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
    let p = obj + 40;
    let f0 = mem.read_f32(p).unwrap();
    cpu.set_t(f0 == cpu.fr[4]); // fcmp/eq fr0,fr4
    if !cpu.t() {
        mem.write_f32(p, cpu.fr[4]).unwrap(); // fmov fr4,@r0
    }

    let r0b = cpu.r[5] & 0xFF; // extu.b r5,r0
    cpu.set_t(r0b == 0); // tst r0,r0
    let r4n = r4 + 92;
    if r0b != 0 {
        let fr0 = mem.read_f32(mem.read_u32(0x8c16ead8).unwrap()).unwrap(); // 0x8c1ffe10
        let fr1 = mem.read_f32(p).unwrap() / fr0; // fmov @r3,fr1 ; fdiv fr0,fr1
        mem.write_f32(obj + 44, fr1).unwrap(); // fmov fr1,@r1
        cpu.fr[0] = fr0;
        cpu.fr[1] = fr1;
        cpu.r[0] = 48;
        cpu.r[1] = 1;
        cpu.r[3] = mem.read_u32(r4n).unwrap();
        cpu.r[4] = r4n;
        mem.write_u8(cpu.r[3] + 48, 1).unwrap(); // rts delay slot
    } else {
        cpu.fr[0] = f0;
        cpu.r[0] = 48;
        cpu.r[1] = 0;
        cpu.r[3] = mem.read_u32(r4n).unwrap();
        cpu.r[4] = r4n;
        mem.write_u8(cpu.r[3] + 48, 0).unwrap();
    }
}
