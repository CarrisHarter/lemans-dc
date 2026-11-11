//! 0x8c0bd614 — element-wise signed min of two 12-word vectors into a third.
//!
//! ```text
//! prologue: push r14,r13,r12 ; add #-48,r15      (frame = sp-60)
//! r3 = @(60,r15)                                 (param_3 = [entry sp])
//! zero 12 words at r15..r15+44
//! six selects at offsets 12,4,8,0,16,20:
//!     r0 = @(off,r5) ; r1 = @(off,r4) ; cmp/gt r1,r0  (T = r0 > r1 signed)
//!     bf  -> r1 = r0 ; bra (delay nop) ; store r1 at frame+off
//! loop (r14 = 0..5, r13 = 0,4,..,20; r4/r5 += 24; r6 = 6):
//!     r7 = @(r13,r5) ; r12 = @(r13,r4) ; cmp/gt r12,r7 (T = r7 > r12)
//!     bf -> r12 = r7 ; store r12 at frame+24+r13 ; r14++
//!     cmp/gt r14,r6 (T = 6 > r14) ; bt.s loop ; delay slot add #4,r13 (always)
//! store local[0..11] to param_3 (r3 += 24 halfway)
//! epilogue: add #48,r15 ; pop r12 ; pop r13 ; rts ; pop r14 (delay)
//! ```
//!
//! Hidden side effects the decompiled C does not show: r4/r5 end at
//! arg+24 (not restored), r3 = param_3+24, r1 = frame+24, r6 = 6,
//! r7 = param_2[11] (last loop load), r0 = local[11], r4 = local[10]
//! (r4 is clobbered by the tail stores), r12/r13/r14 are restored from
//! the stack (so the loop counter in r14/r13 is discarded), r15 ends
//! back at the entry sp, and T is 0 on exit (6 > 6 is false).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[15];
    let saved12 = cpu.r[12];
    let saved13 = cpu.r[13];
    let saved14 = cpu.r[14];
    cpu.r[15] = s.wrapping_sub(60);
    let frame = cpu.r[15];
    let p3 = mem.read_u32(frame + 60).unwrap(); // param_3 = [entry sp]

    let mut local = [0u32; 12];

    // Six scalar min selects; store targets are frame+12, +4, +8, +0, +16, +20.
    let order = [12u32, 4, 8, 0, 16, 20];
    for &off in &order {
        let r0 = mem.read_u32(cpu.r[5].wrapping_add(off)).unwrap(); // param_2
        let mut r1 = mem.read_u32(cpu.r[4].wrapping_add(off)).unwrap(); // param_1
        // cmp/gt r1,r0 -> T = (r0 > r1) signed; bf (T==0) -> mov r0,r1
        if !((r0 as i32) > (r1 as i32)) {
            r1 = r0;
        }
        local[(off / 4) as usize] = r1;
    }

    // Loop over offsets 24..44 of both vectors.
    let mut r4 = cpu.r[4].wrapping_add(24);
    let mut r5 = cpu.r[5].wrapping_add(24);
    cpu.r[6] = 6;
    let r1 = frame + 24;
    let mut r13 = 0u32;
    let mut r14 = 0u32;
    let mut last_r7 = 0u32;
    loop {
        let r7 = mem.read_u32(r5.wrapping_add(r13)).unwrap();
        let mut r12 = mem.read_u32(r4.wrapping_add(r13)).unwrap();
        last_r7 = r7;
        // cmp/gt r12,r7 -> T = (r7 > r12) signed; bf (T==0) -> mov r7,r12
        if !((r7 as i32) > (r12 as i32)) {
            r12 = r7;
        }
        local[(6 + (r13 / 4)) as usize] = r12;
        r14 = r14.wrapping_add(1);
        // cmp/gt r14,r6 -> T = (r6 > r14); bt.s back; delay slot r13 += 4 always
        let t = (cpu.r[6] as i32) > (r14 as i32);
        r13 = r13.wrapping_add(4);
        cpu.set_t(t);
        if !t {
            break;
        }
    }

    // Copy local[0..11] to param_3, in order.
    for i in 0..12u32 {
        let _ = mem.write_u32(p3.wrapping_add(4 * i), local[i as usize]);
    }

    // Register state after the tail stores and the epilogue pops.
    cpu.r[0] = local[11];
    cpu.r[1] = r1;
    cpu.r[3] = p3.wrapping_add(24);
    cpu.r[4] = local[10];
    cpu.r[5] = r5;
    cpu.r[7] = last_r7;
    cpu.r[12] = saved12;
    cpu.r[13] = saved13;
    cpu.r[14] = saved14;
    cpu.r[15] = s;
}
