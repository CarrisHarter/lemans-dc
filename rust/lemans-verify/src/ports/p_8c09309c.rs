//! 0x8c09309c — same shape as 0x8c093070 but the flag byte lands at r4+0x5f
//! (literal 95) instead of r4+0x60.
//!
//! Original:
//! ```text
//! mov.l 0x8c0930bc,r0 ; ! 8c2ab460 ; mov.l @r0,r1
//! mov.l 0x8c0930c0,r0 ; ! 345 ; mov.b @(r0,r1),r0 ; extu.b x2
//! tst r0,r0 ; bf 0x8c0930b2
//! extu.b r5,r0 ; tst r0,r0 ; bt 0x8c0930b8
//! mov.l 0x8c0930c4,r0 ; ! 95 ; mov #1,r1 ; mov.b r1,@(r0,r4) ; rts ; nop
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = mem.read_u32(mem.read_u32(0x8c0930bc).unwrap_or(0)).unwrap_or(0);
    let off = mem.read_u32(0x8c0930c0).unwrap_or(0); // 0x345
    let b = mem.read_u8(p.wrapping_add(off)).unwrap_or(0) as u32;
    cpu.set_t(b == 0);
    if b == 0 {
        let r0 = cpu.r[5] & 0xFF;
        cpu.set_t(r0 == 0);
        if r0 == 0 {
            cpu.r[0] = r0;
            cpu.r[1] = p;
            return; // rts ; nop, T=1
        }
    }
    cpu.r[0] = mem.read_u32(0x8c0930c4).unwrap_or(0); // 95
    cpu.r[1] = 1;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), 1);
    // rts ; nop — T=0
}
