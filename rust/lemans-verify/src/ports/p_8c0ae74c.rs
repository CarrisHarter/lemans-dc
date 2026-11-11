//! 0x8c0ae74c — compute the byte offset of the `r5`-th table entry (mode 1
//! walks length-prefixed groups, mode 0 adds 3 per step) and write
//! `{base, count, offset, mode}` to the struct pointed at by `[r15]`.
//!
//! Original:
//! ```text
//! mov.l r14,@-r15 ; add #-16,r15 ; mov.l @(20,r15),r14   ; r14 = [entry_r15]
//! mov.l @(8,r4),r6 ; mov #0,r7 ; mov.l @(4,r4),r3 ; mov r7,r1 ; mov.l @r4,r0
//! cmp/pl r5 ; bf.s done ; mov.l r0,@(12,r15)              ; local = mode
//! loop: mov.l @(12,r15),r4 ; tst r4,r4 ; bt is0 ; mov r4,r0 ; cmp/eq #1,r0
//! bt is1 ; bra next ; nop
//! is0: bra next ; add #3,r7
//! is1: mov r7,r0 ; shll r0 ; mov.w @(r0,r3),r0 ; extu.w r0,r0 ; add #1,r0 ; add r0,r7
//! next: add #1,r1 ; cmp/gt r1,r5 ; bt loop
//! done: mov.l r3,@r14 ; mov.l r6,@(4,r14) ; mov.l r7,@(8,r14)
//! mov.l @(12,r15),r0 ; mov.l r0,@(12,r14) ; add #16,r15 ; rts ; mov.l @r15+,r14
//! ```
//! Side effects the C hides: `r4` is clobbered with the saved mode on every
//! loop iteration, `r0` returns the mode (not a pointer), `r1` = iteration
//! count, `r3` = table base, `r6` = count, `r7` = accumulated offset, and the
//! out pointer comes from `[r15]` at entry (a stack slot here). T ends 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let out = mem.read_u32(cpu.r[15]).unwrap_or(0); // [entry r15], after push + add #-16
    let r6 = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    cpu.r[6] = r6;
    cpu.r[7] = 0;
    let r3 = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = r3;
    cpu.r[1] = 0;
    let mode = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mode;
    cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
    if (cpu.r[5] as i32) > 0 {
        loop {
            cpu.r[4] = mode; // mov.l @(12,r15),r4 reloads the saved mode
            if mode == 0 {
                cpu.r[7] = cpu.r[7].wrapping_add(3);
            } else if mode == 1 {
                let len = mem
                    .read_u16(cpu.r[7].wrapping_mul(2).wrapping_add(r3))
                    .unwrap_or(0) as u32;
                cpu.r[7] = cpu.r[7].wrapping_add(len.wrapping_add(1));
            }
            cpu.r[1] = cpu.r[1].wrapping_add(1);
            let t = (cpu.r[5] as i32) > (cpu.r[1] as i32);
            cpu.set_t(t);
            if !t {
                break;
            }
        }
    }
    let _ = mem.write_u32(out, r3);
    let _ = mem.write_u32(out.wrapping_add(4), cpu.r[6]);
    let _ = mem.write_u32(out.wrapping_add(8), cpu.r[7]);
    cpu.r[0] = mode; // mov.l @(12,r15),r0
    let _ = mem.write_u32(out.wrapping_add(12), cpu.r[0]);
}
