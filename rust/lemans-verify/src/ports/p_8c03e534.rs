//! 0x8c03e534 — find a slot in a ring of 17-float records and copy r5's
//! record into it. Returns 1 on success, 0 otherwise.
//!
//! Paths: (a) flag byte at +0x44 zero and count(+0x3c) <= limit(+0x40) ->
//! return 0 immediately; (b) flag nonzero -> blind copy of 17 u32 into
//! [r4+0x38], clear the flag byte, return 1; (c) otherwise scan slots from
//! [r4+0x80] (stride 68) for the first whose float is >= the constant at
//! 0x8c03e630 (0.95f), wrapping to [r4+0x38] if the scan reaches the current
//! pointer. Hidden side effects: the scan pointers r3/r6/r7 and r5 (advanced
//! by 68) stay live in registers, fr0/fr1 hold the constant and last slot
//! value, [r4+0x80] is updated (and reset to base when it hits the end), and
//! T is whatever the last cmp/eq or cmp/hi left.

use sh4_oracle::{Cpu, Memory};

const CONST_ADDR: u32 = 0x8c03e630; // 0.95f
const SLOT_STRIDE: u32 = 68;

/// Copy 17 u32 from `src` into `slot`; sets r3 to the final dst pointer.
fn copy_record(cpu: &mut Cpu, mem: &mut Memory, src: &mut u32, slot: u32) -> u32 {
    let mut dst = slot;
    let mut r0 = 8u32;
    loop {
        let v = mem.read_u32(*src).unwrap_or(0);
        *src = src.wrapping_add(4);
        r0 = r0.wrapping_sub(1);
        cpu.set_t(r0 == 0); // dt r0
        let _ = mem.write_u32(dst, v);
        let v = mem.read_u32(*src).unwrap_or(0);
        *src = src.wrapping_add(4);
        let _ = mem.write_u32(dst.wrapping_add(4), v);
        dst = dst.wrapping_add(8); // bf.s delay slot, both paths
        if r0 == 0 {
            break;
        }
    }
    let v = mem.read_u32(*src).unwrap_or(0);
    *src = src.wrapping_add(4);
    let _ = mem.write_u32(dst, v);
    cpu.r[3] = dst;
    dst
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    let mut r5 = cpu.r[5];

    let b44 = mem.read_u8(s.wrapping_add(0x44)).unwrap_or(0);
    cpu.r[6] = b44 as u32; // mov.b (sign-ext) + extu.b r6,r6
    let mut r3: u32 = 1;
    if b44 == 0 {
        let r1 = mem.read_u32(s.wrapping_add(0x40)).unwrap_or(0);
        let r0 = mem.read_u32(s.wrapping_add(0x3c)).unwrap_or(0);
        cpu.r[1] = r1;
        if (r0 as i32) <= (r1 as i32) {
            r3 = 0; // cmp/gt not taken -> mov #0,r3
        }
    }
    cpu.r[3] = r3;
    cpu.set_t(r3 == 0); // extu.b r3,r0 ; tst r0,r0
    if r3 == 0 {
        cpu.r[0] = 0; // bra delay slot
        return;
    }

    if b44 != 0 {
        // blind copy path
        let mut r6 = mem.read_u32(s.wrapping_add(0x38)).unwrap_or(0);
        let mut r0 = 8u32;
        let mut last = 0u32;
        loop {
            last = mem.read_u32(r5).unwrap_or(0);
            r5 = r5.wrapping_add(4);
            r0 = r0.wrapping_sub(1);
            cpu.set_t(r0 == 0); // dt r0
            let _ = mem.write_u32(r6, last);
            last = mem.read_u32(r5).unwrap_or(0);
            r5 = r5.wrapping_add(4);
            let _ = mem.write_u32(r6.wrapping_add(4), last);
            r6 = r6.wrapping_add(8); // bf.s delay slot, both paths
            if r0 == 0 {
                break;
            }
        }
        last = mem.read_u32(r5).unwrap_or(0);
        r5 = r5.wrapping_add(4);
        let _ = mem.write_u32(r6, last);
        let _ = mem.write_u8(s.wrapping_add(0x44), 0);
        cpu.r[0] = 1; // bra delay slot
        cpu.r[1] = 0;
        cpu.r[3] = last;
        cpu.r[5] = r5;
        cpu.r[6] = r6;
        cpu.set_t(true); // last dt hit zero
        return;
    }

    // scan path: byte44 == 0 and count > limit
    let count = mem.read_u32(s.wrapping_add(0x3c)).unwrap_or(0);
    let base = mem.read_u32(s.wrapping_add(0x38)).unwrap_or(0);
    let end = base.wrapping_add((count << 4).wrapping_add(count) << 2);
    let cur = mem.read_u32(s.wrapping_add(0x80)).unwrap_or(0);
    cpu.r[1] = count;
    cpu.r[3] = cur;
    cpu.set_t(end > cur); // cmp/hi r3,r7 -> T = r7 > r3
    cpu.r[6] = cur; // bf.s delay slot: mov r3,r6 (both paths)
    let mut r6 = cur;

    let fr0 = mem.read_f32(CONST_ADDR).unwrap_or(0.0);
    let mut found: Option<u32> = None;
    if end > cur {
        cpu.fr[0] = fr0; // mova + fmov at loop entry
        loop {
            let fr1 = mem.read_f32(r6).unwrap_or(0.0);
            cpu.fr[1] = fr1;
            let free = fr0 > fr1; // fcmp/gt
            cpu.set_t(free);
            if !free {
                found = Some(r6);
                break;
            }
            r6 = r6.wrapping_add(SLOT_STRIDE);
            cpu.r[6] = r6;
            let more = end > r6; // cmp/hi r6,r7 -> T = r7 > r6
            cpu.set_t(more);
            if !more {
                break;
            }
        }
    }

    if let Some(slot) = found {
        copy_record(cpu, mem, &mut r5, slot);
        let newcur = slot.wrapping_add(SLOT_STRIDE);
        let _ = mem.write_u32(s.wrapping_add(0x80), newcur);
        let mut r1 = mem.read_u32(s.wrapping_add(0x80)).unwrap_or(0);
        cpu.set_t(end == r1); // cmp/eq r1,r7
        if end == r1 {
            r1 = base;
            let _ = mem.write_u32(s.wrapping_add(0x80), r1);
        }
        cpu.r[0] = 1; // bra delay slot
        cpu.r[1] = r1;
        cpu.r[5] = r5;
        cpu.r[6] = newcur;
        cpu.r[7] = end;
        return;
    }

    // second pass from base while cur > p
    cpu.set_t(cur > base); // cmp/hi r14,r3 -> T = r3 > r14
    if !(cur > base) {
        cpu.r[0] = 0; // mov #0,r0 at 0x8c03e624
        cpu.r[3] = cur;
        cpu.r[5] = r5;
        cpu.r[6] = r6;
        cpu.r[7] = end;
        return; // T = false
    }
    cpu.fr[0] = fr0; // mova + fmov at 0x8c03e5dc
    let mut p = base;
    found = None;
    loop {
        let fr1 = mem.read_f32(p).unwrap_or(0.0);
        cpu.fr[1] = fr1;
        let free = fr0 > fr1;
        cpu.set_t(free);
        if !free {
            found = Some(p);
            break;
        }
        p = p.wrapping_add(SLOT_STRIDE);
        let more = cur > p; // cmp/hi r14,r3 -> T = r3 > r14
        cpu.set_t(more);
        if !more {
            break;
        }
    }

    if let Some(slot) = found {
        copy_record(cpu, mem, &mut r5, slot);
        let newcur = slot.wrapping_add(SLOT_STRIDE);
        let _ = mem.write_u32(s.wrapping_add(0x80), newcur);
        cpu.r[5] = base; // reloaded from [r4+0x38]
        let r1 = mem.read_u32(s.wrapping_add(0x80)).unwrap_or(0);
        cpu.r[1] = r1;
        cpu.r[3] = end; // base + 68*count recomputed
        cpu.set_t(end == r1); // cmp/eq r1,r3
        if end == r1 {
            let _ = mem.write_u32(s.wrapping_add(0x80), base);
            cpu.r[1] = base;
        }
        cpu.r[0] = 1; // bra delay slot
        cpu.r[6] = r6;
        cpu.r[7] = end;
        return;
    }

    // nothing found anywhere: return 0
    cpu.r[0] = 0;
    cpu.r[3] = cur;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.r[7] = end;
    // T = false from the last cmp/hi (cur > p)
    cpu.set_t(false);
}
