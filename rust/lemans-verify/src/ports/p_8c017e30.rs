//! 0x8c017e30 — copies two descending word runs from a source pointer into
//! the object at r4 and returns one final word.
//!
//! Side effects the C hides: r3 = 0 or 16 from the [r4+0xb0] test; the
//! `bt.s` delay slot `add r6,r5` (r6 = -4) runs on BOTH paths; loop 1 runs
//! only when bit6 of the byte at [mem32[r5+4]] is set; loop 2 resets its
//! counter (`mov #0,r11` at 0x8c017e8a); r7 = count2*4; r1 = -4; final
//! T = 0 from loop 2's last cmp/gt. Stores: count1 words at
//! r4+92+(15-k)*4, count2 words at r4+28+(14-j)*4, and [r4+20] =
//! mem32[r4+24] + r1_original. r11/r12/r13/r14 are pushed on entry and popped
//! by the epilogue, so the caller sees its original values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r1 = mem.read_u32(r4 + 0xa0).unwrap_or(0);
    let v_b0 = mem.read_u32(r4 + 0xb0).unwrap_or(0);
    let r3 = if v_b0 == 0 { 0 } else { 16 };
    let byte_ptr = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    let mut r5 = mem
        .read_u32(r4 + 24)
        .unwrap_or(0)
        .wrapping_add(r1)
        .wrapping_sub(r3);
    let flags = u32::from(mem.read_u8(byte_ptr).unwrap_or(0)) & 0x40;
    // bt.s delay slot: add r6,r5 with r6 = 0xfffffffc — executes on both paths
    r5 = r5.wrapping_add(0xffff_fffc);
    let mut r11: u32 = 0; // loop counter; popped at the epilogue, kept in a local
    if flags != 0 {
        let dst1 = r4 + 92;
        loop {
            let count1 = mem.read_u32(r4 + 0xac).unwrap_or(0);
            if !((count1 as i32) > (r11 as i32)) {
                break;
            }
            let idx = 15u32.wrapping_sub(r11) << 2;
            let w = mem.read_u32(r5).unwrap_or(0);
            r11 = r11.wrapping_add(1);
            let _ = mem.write_u32(dst1.wrapping_add(idx), w);
            r5 = r5.wrapping_add(0xffff_fffc);
        }
    }
    let dst2 = r4 + 28;
    r11 = 0; // mov #0,r11 at 0x8c017e8a — loop 2 restarts the counter
    let mut count2: u32;
    loop {
        count2 = mem.read_u32(r4 + 0xa8).unwrap_or(0);
        if !((count2 as i32) > (r11 as i32)) {
            break;
        }
        let idx = 14u32.wrapping_sub(r11) << 2;
        let w = mem.read_u32(r5).unwrap_or(0);
        r11 = r11.wrapping_add(1);
        let _ = mem.write_u32(dst2.wrapping_add(idx), w);
        r5 = r5.wrapping_add(0xffff_fffc);
    }
    let r6 = mem.read_u32(r4 + 24).unwrap_or(0).wrapping_add(r1);
    let count1 = mem.read_u32(r4 + 0xac).unwrap_or(0);
    let r5f = r6
        .wrapping_sub(r3)
        .wrapping_add(0xffff_fffc)
        .wrapping_sub(count1 << 2)
        .wrapping_sub(count2 << 2);
    cpu.r[0] = mem.read_u32(r5f).unwrap_or(0);
    let _ = mem.write_u32(r4 + 20, r6);
    cpu.r[1] = 0xffff_fffc;
    cpu.r[3] = r3;
    cpu.r[5] = r5f;
    cpu.r[6] = r6;
    cpu.r[7] = count2 << 2; // shll2 r7 of the count reloaded by the last check
    cpu.set_t(false); // loop 2's final cmp/gt exits with T = 0
}
