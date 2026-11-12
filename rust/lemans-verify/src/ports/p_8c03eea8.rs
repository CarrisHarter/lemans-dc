//! 0x8c03eea8 — shift a four-slot, 100-byte-stride record array down by one
//! and insert a new record into slot 0.
//!
//! The array base is `r4 + 64`. For slots 3, 2, 1 the routine moves the
//! previous slot's `strcpy` string (callee 0x8c017f48), its 16-byte field at
//! +0x50 and the word at -4. Then it fills slot 0: `strcpy(r4+64, r5)`, the
//! four stack words into `r4+0x90`, `r6` into `r4+0x3c`, and zero into
//! `r4+0x1cc` (slot 3's last word).
//!
//! Side effects the C hides: the prologue `mul.l` and each loop `mul.l` write
//! MACH/MACL (the last one leaves both zero); every callee leaves `r2 = 0`,
//! `r4`/`r5` one past the copied NUL and T set; and the pushed `r8`-`r14` and
//! `pr` are restored, so `r6` ends as the entry stack pointer, `r7` as
//! `r4+0x90`, `r0` as 0x1cc and `r3` as the third stack word.
//!
//! Two data-layout hazards are load-bearing. The slot word write at slot-4
//! runs before the string copy, so it pre-clobbers source bytes 96..99 of the
//! next slot; and the callee copies forward with destination `src+100`, so a
//! string of 100 or more bytes overwrites its own terminator and never stops.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = cpu.r[4];
    let src = cpu.r[5];
    let arg3 = cpu.r[6];
    let entry_sp = cpu.r[15];

    // Literal pool at 0x8c03ef64.
    let stride = mem.read_u32(0x8c03ef64).unwrap_or(0); // 300
    let field = mem.read_u32(0x8c03ef68).unwrap_or(0); // 144
    let last = mem.read_u32(0x8c03ef70).unwrap_or(0); // 460

    // Prologue: push r14..r8, then pr, then spill r6.
    for i in (8..=14).rev() {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let v = cpu.r[i];
        let _ = mem.write_u32(cpu.r[15], v);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], arg3);

    cpu.mach = 0;
    cpu.macl = 100 * 3; // mul.l r1,r0 (mov #100,r1 ; mov #3,r0)
    let mut i = 3u32; // r14
    let mut off = stride; // r13, -100 per pass
    let mut shift = p.wrapping_add(300); // r12, -100 per pass (delay slot)

    while i > 0 {
        // Word at -0x28 -> +0x3c for this slot.
        let word = mem
            .read_u32(p.wrapping_add(off).wrapping_sub(40))
            .unwrap_or(0);
        let _ = mem.write_u32(p.wrapping_add(off).wrapping_add(60), word);

        // 16-byte field at +0x2c -> +0x90 for this slot.
        let src_field = p.wrapping_add(off).wrapping_add(44);
        let dst_field = p.wrapping_add(off).wrapping_add(field);
        let w0 = mem.read_u32(src_field).unwrap_or(0);
        let w1 = mem.read_u32(src_field.wrapping_add(4)).unwrap_or(0);
        let _ = mem.write_u32(dst_field, w0);
        let _ = mem.write_u32(dst_field.wrapping_add(4), w1);
        let w2 = mem.read_u32(src_field.wrapping_add(8)).unwrap_or(0);
        let _ = mem.write_u32(dst_field.wrapping_add(8), w2);
        let w3 = mem.read_u32(src_field.wrapping_add(12)).unwrap_or(0);
        let _ = mem.write_u32(dst_field.wrapping_add(12), w3);

        // strcpy(r12+64, r4 + 100*(r14-1) + 64): the previous slot's string.
        cpu.mach = 0;
        cpu.macl = 100 * (i - 1);
        cpu.r[4] = shift.wrapping_add(64);
        cpu.r[5] = p.wrapping_add(100 * (i - 1)).wrapping_add(64);
        cpu.pr = 0x8c03ef18;
        crate::registry::p_8c017f48::run(cpu, mem);

        i -= 1; // add #-1,r14
        off = off.wrapping_sub(100); // add #-100,r13
        shift = shift.wrapping_sub(100); // add #-100,r12 (bt.s delay slot)
    }

    // The spill slot is still live; re-read it like the original does.
    let arg3 = mem.read_u32(cpu.r[15]).unwrap_or(arg3);
    let _ = mem.write_u32(p.wrapping_add(60), arg3);

    // r6 = frame top + 36 = entry stack pointer; the stack words live there.
    let mut stack = [0u32; 4];
    for (k, slot) in stack.iter_mut().enumerate() {
        *slot = mem
            .read_u32(entry_sp.wrapping_add(4 * k as u32))
            .unwrap_or(0);
    }
    let mut r7 = p.wrapping_add(field);
    for value in stack {
        let _ = mem.write_u32(r7, value);
        r7 = r7.wrapping_add(4);
    }

    // Insert the new slot-0 string.
    cpu.r[4] = p.wrapping_add(64);
    cpu.r[5] = src;
    cpu.pr = 0x8c03ef4a;
    crate::registry::p_8c017f48::run(cpu, mem);

    let _ = mem.write_u32(p.wrapping_add(last), 0);
    cpu.r[0] = last;
    cpu.r[1] = 0;
    cpu.r[3] = stack[2];
    cpu.r[6] = entry_sp;
    cpu.r[7] = p.wrapping_add(field);

    // Epilogue: drop the spill slot, pop pr, r8..r13 and r14 in the delay slot.
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for i in 8..=13 {
        cpu.r[i] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
