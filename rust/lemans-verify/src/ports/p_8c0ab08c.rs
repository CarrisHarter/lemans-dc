//! 0x8c0ab08c — set field 0x28 and flag byte 0xc of up to 27 objects in the
//! pointer array at `[r4]`, then store `r5` into `r4->field4`.
//!
//! Original:
//! ```text
//! mov.l r14,@-r15 ; mov.l @(4,r4),r0 ; cmp/eq r0,r5 ; bt 0x8c0ab0bc
//! mov #0,r7 ; mov.l r5,@(4,r4) ; mov r7,r14 ; mov #1,r3 ; mov #27,r1
//! loop: mov.l @r4,r6 ; mov r14,r0 ; mov.l @(r0,r6),r6 ; tst r6,r6 ; bt skip
//! mov.l @(40,r6),r0 ; cmp/eq r0,r5 ; bt skip ; mov.l r5,@(40,r6)
//! mov r3,r0 ; mov.b r0,@(12,r6)
//! skip: add #1,r7 ; cmp/gt r7,r1 ; bt.s loop ; add #4,r14
//! rts ; mov.l @r15+,r14
//! ```
//! Side effects the C hides: `r4->field4` is stored even when objects are
//! skipped, `r0` keeps the last tested/stored value (1 after a store), `r1`
//! ends 27, `r3` = 1, `r6` = last array entry, `r7` = 27, `r14` = 108 on the
//! loop path (seed on the early-out), and T ends 0 after 27 iterations (1 on
//! the early-out).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r0 = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[0] = r0;
    cpu.set_t(r0 == cpu.r[5]);
    if r0 == cpu.r[5] {
        return; // bt to rts; r14 pop restores the seed
    }
    cpu.r[7] = 0;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[5]);
    let mut r14: u32 = 0;
    cpu.r[3] = 1;
    cpu.r[1] = 27;
    loop {
        let r6 = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = r14;
        let r6 = mem.read_u32(r14.wrapping_add(r6)).unwrap_or(0);
        cpu.r[6] = r6;
        if r6 != 0 {
            let v = mem.read_u32(r6.wrapping_add(40)).unwrap_or(0);
            cpu.r[0] = v;
            if v != cpu.r[5] {
                let _ = mem.write_u32(r6.wrapping_add(40), cpu.r[5]);
                cpu.r[0] = cpu.r[3];
                let _ = mem.write_u8(r6.wrapping_add(12), cpu.r[0] as u8);
            }
        }
        cpu.r[7] = cpu.r[7].wrapping_add(1);
        let t = (cpu.r[1] as i32) > (cpu.r[7] as i32);
        cpu.set_t(t);
        if !t {
            break;
        }
        r14 = r14.wrapping_add(4); // bt.s delay slot
    }
}
