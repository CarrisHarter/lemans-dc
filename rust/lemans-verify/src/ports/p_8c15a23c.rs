//! 0x8c15a23c — find_block_by_size.
//!
//! Walks the block list at `[[0x8c2b1ef8]]+8`, advancing by `|size|+8` per node,
//! and returns the first node whose size is greater than the running total.
//! A size of `0x7fffffff` stops the walk and returns the previous total.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c15a274).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[1] + 8).unwrap_or(0);
    let mut r0: u32 = 0;
    let mut r1 = cpu.r[1];
    let mut r3 = cpu.r[3];
    let mut r5: u32 = cpu.r[5]; // the early bt.s path never loads r5
    let t = r3 == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = 0; // delay slot of `bt.s`
    } else {
        let r4: u32 = 0x7fffffff;
        r1 = r0; // 0x8c15a24a: mov r0,r1 runs once, before the loop head
        loop {
            r5 = mem.read_u32(r3 + 4).unwrap_or(0);
            let gt = (r5 as i32) > (r0 as i32);
            cpu.set_t(gt);
            if gt {
                let eq = r5 == r4;
                cpu.set_t(eq);
                if eq {
                    r3 = r1; // delay slot of `bra`
                } else {
                    r3 = r3.wrapping_add(r5);
                    r0 = r5;
                    r3 = r3.wrapping_add(8); // delay slot of `bra`
                }
            } else {
                let pz = (r5 as i32) >= 0;
                cpu.set_t(pz);
                if !pz {
                    r5 = r5.wrapping_neg();
                }
                r3 = r3.wrapping_add(r5);
                r3 = r3.wrapping_add(8);
            }
            let t2 = r3 == 0;
            cpu.set_t(t2);
            if !t2 {
                continue;
            }
            break;
        }
        cpu.r[4] = r4;
    }
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[5] = r5;
}
