//! 0x8c194e84 — pick a pointer through one of four pool entries.
//!
//! ```text
//! mov.b @(2,r4),r0 ; extu.b ; tst ; bt 0x8c194e92
//! mov.l 0x8c194ec8,r0 (-> 0x8c2b86e8) ; rts ; mov.l @r0,r0   ; byte2 != 0
//! 0x8c194e92: mov.b @(1,r4),r0 ; extu.b ; tst ; bt 0x8c194ec2
//! rts ; mov #0,r0                                             ; byte1 == 0
//! mov r4,r0 ; add #4,r0 ; fmov @r0,fr1 ; fldi0 fr0 ; fcmp/gt fr0,fr1
//! bf 0x8c194ebc -> pool 0x8c194ed4 (-> 0x8c2b86b8)            ; !(fr1 > 0)
//! mov.b @(12,r4),r0 ; extu.b ; tst ; bt 0x8c194eb4
//! pool 0x8c194ecc (-> 0x8c2b86c0) ; rts ; mov.l @r0,r0         ; byte12 != 0
//! 0x8c194eb4: pool 0x8c194ed0 (-> 0x8c2b86c8) ; mov.l @r0,r0 ; rts ; nop
//! ```
//! `fcmp/gt fr0,fr1` sets T = (fr1 > fr0). T at exit comes from the last
//! `tst`/`fcmp` on the path taken.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let b2 = mem.read_u8(r4 + 2).unwrap() as u32; // mov.b @(2,r4),r0 ; extu.b
    cpu.r[0] = b2;
    cpu.set_t(b2 == 0); // tst r0,r0
    if b2 != 0 {
        let p = mem.read_u32(0x8C194EC8).unwrap();
        cpu.r[0] = mem.read_u32(p).unwrap(); // rts delay slot
        return;
    }
    let b1 = mem.read_u8(r4 + 1).unwrap() as u32;
    cpu.r[0] = b1;
    cpu.set_t(b1 == 0);
    if b1 == 0 {
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        return;
    }
    cpu.r[0] = r4.wrapping_add(4);
    cpu.fr[1] = mem.read_f32(r4 + 4).unwrap();
    cpu.fr[0] = 0.0; // fldi0
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1 -> T = fr1 > fr0
    if cpu.fr[1] <= cpu.fr[0] {
        let p = mem.read_u32(0x8C194ED4).unwrap();
        cpu.r[0] = mem.read_u32(p).unwrap();
        return;
    }
    let b12 = mem.read_u8(r4 + 12).unwrap() as u32;
    cpu.r[0] = b12;
    cpu.set_t(b12 == 0);
    let pool = if b12 == 0 { 0x8C194ED0 } else { 0x8C194ECC };
    let p = mem.read_u32(pool).unwrap();
    cpu.r[0] = mem.read_u32(p).unwrap();
}
