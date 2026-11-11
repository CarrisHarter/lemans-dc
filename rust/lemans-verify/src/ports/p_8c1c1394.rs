//! 0x8c1c1394 — scan the NUL-separated buffer at `*0x8c212c90`, counting the
//! skips past zero bytes; returns that count in `r0`.
//!
//! Original:
//! ```text
//! r4 = 0x8c212c90 ; r3 = mov.b @r4 ; tst r3,r3 ; bt.s rts ; mov #0,r5 (delay)
//! loop: mov.b @r4+,r3 ; tst r3,r3 ; bf loop
//!       add #1,r4 ; mov.b @r4,r3 ; tst r3,r3 ; bf.s loop ; add #1,r5 (delay)
//! rts ; mov r5,r0   (delay)
//! ```
//! `mov.b` sign-extends; `tst r3,r3` is true exactly for a zero byte. T ends 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = mem.read_u32(0x8c1c13ec).unwrap_or(0);
    cpu.r[4] = r4;
    let mut r3 = mem.read_u8(r4).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[3] = r3;
    cpu.set_t(r3 == 0);
    cpu.r[5] = 0; // bt.s delay slot runs either way
    if r3 != 0 {
        loop {
            // 0x8c1c139e
            r3 = mem.read_u8(r4).unwrap_or(0) as i8 as i32 as u32;
            r4 = r4.wrapping_add(1);
            cpu.r[4] = r4;
            cpu.r[3] = r3;
            cpu.set_t(r3 == 0);
            if r3 != 0 {
                continue; // bf loop
            }
            // 0x8c1c13a4
            r4 = r4.wrapping_add(1);
            cpu.r[4] = r4;
            r3 = mem.read_u8(r4).unwrap_or(0) as i8 as i32 as u32;
            cpu.r[3] = r3;
            cpu.set_t(r3 == 0);
            cpu.r[5] = cpu.r[5].wrapping_add(1); // bf.s delay slot runs either way
            if r3 != 0 {
                continue; // bf.s loop
            }
            break;
        }
    }
    // rts delay slot
    cpu.r[0] = cpu.r[5];
}
