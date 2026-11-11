//! 0x8c0e7ab4 — memset-like fill: write byte r6 to r5 bytes at r4, then a
//! NUL terminator at r4+r5. Three paths: count <= 0 (only the terminator),
//! 0 < count <= 2 (tail loop only), count > 2 (2x-unrolled loop then tail).
//!
//! ```text
//! cmp/pl r5; bf.s done; mov #0,r3 (delay, always)
//! r1 = [0x8c0e7aec] = -2; r0 = 2; r7 = r5
//! cmp/gt r0,r5; bf.s tail; add r1,r7 (delay, always: r7 = r5-2)
//! unrolled: r0=r3; [r4+r0]=r6; r3+=2; r0+=1; T=(r7>r3); bt.s unrolled;
//!   [r4+r0]=r6 (delay, always); bra tail; nop
//! tail: r0=r3; [r4+r0]=r6; r3+=1; T=(r5>r3); bt tail
//! done: r0=r5; r1=0; rts; [r4+r0]=r1 (delay: NUL terminator)
//! ```
//! Hidden side effects the C does not show: r3 ends at r5 (or 0 when r5<=0),
//! r7 = r5-2 on the positive paths, r0 = r5, r1 = 0, final T = 0, and both
//! .s delay slots run whether or not their branch is taken.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
    cpu.r[3] = 0; // bf.s delay slot: always runs
    if (cpu.r[5] as i32) > 0 {
        cpu.r[1] = mem.read_u32(0x8C0E7AEC).unwrap(); // 0xfffffffe = -2
        cpu.r[0] = 2;
        cpu.r[7] = cpu.r[5];
        cpu.set_t((cpu.r[5] as i32) > (cpu.r[0] as i32)); // cmp/gt r0,r5
        cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[1]); // bf.s delay slot: always
        if (cpu.r[5] as i32) > 2 {
            // unrolled loop at 0x8c0e7ac6
            loop {
                cpu.r[0] = cpu.r[3];
                let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[6] as u8);
                cpu.r[3] = cpu.r[3].wrapping_add(2);
                cpu.r[0] = cpu.r[0].wrapping_add(1);
                cpu.set_t((cpu.r[7] as i32) > (cpu.r[3] as i32)); // cmp/gt r3,r7
                // bt.s delay slot: always runs, even when not taken
                let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[6] as u8);
                if !cpu.t() {
                    break;
                }
            }
            // bra 0x8c0e7ade; nop
        }
        // tail loop at 0x8c0e7ad8
        loop {
            cpu.r[0] = cpu.r[3];
            let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[6] as u8);
            cpu.r[3] = cpu.r[3].wrapping_add(1);
            cpu.set_t((cpu.r[5] as i32) > (cpu.r[3] as i32)); // cmp/gt r3,r5
            if !cpu.t() {
                break;
            }
        }
    }
    // 0x8c0e7ae2: done
    cpu.r[0] = cpu.r[5];
    cpu.r[1] = 0;
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8); // rts delay slot
}
