//! 0x8c0802f0 — destroy the tail of a ring range and free its elements.
//!
//! `r4` is the ring descriptor: `+4` capacity, `+8` count, `+12` start index,
//! `+16` item array. With `count == 0` it does nothing. Otherwise it builds
//! `end = start + count`; the end pointer wraps by `capacity` when the sum
//! exceeds it (`cmp/hi` picks between the two `store_three` calls). The end
//! iterator `{descriptor+4, count, items+end*4}` is copied to the frame and
//! decremented once by 0x8c079c78, which yields `count-1` in the counter and
//! selects the last element pointer (wrapping from `items` to the array end
//! when it sits on the base). A second iterator `{descriptor+4, 0,
//! items+start*4}` is built and copied. The loop then frees `count-1` elements
//! starting at `start`, advancing the ring pointer and wrapping it to `items`
//! at `items+capacity*4`. Exits with the compiler's `T=1` residue.
//!
//! The `jsr` delay slots are the `add` that lands each destination on the frame
//! slot used by the next call (`+12`, `+36`, `+24`), and the `bt.s`/`bf.s`
//! delay slots (`mov.l @(32,r15),r11`, `add #1,r12`) always execute.

use sh4_oracle::{Cpu, Memory};

const STORE_THREE: u32 = 0x8c079cac;
const COPY_THREE: u32 = 0x8c079c9c;
const DEC_FIELD: u32 = 0x8c079c78;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0802f0..0x8c0802fa: push r14,r13,r12,r11,pr and reserve 48 bytes.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(48);

    cpu.r[11] = cpu.r[4]; // mov r4,r11
    cpu.r[6] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[6] == 0); // tst r6,r6
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(cpu.r[11].wrapping_add(12)).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]); // end = start + count
        cpu.set_t(cpu.r[1] > cpu.r[3]); // cmp/hi r3,r1
        if cpu.t() {
            // 0x8c08030e..0x8c08031e: end fits, point at items+end*4.
            cpu.r[7] = mem.read_u32(cpu.r[11].wrapping_add(16)).unwrap_or(0);
            cpu.r[5] = cpu.r[11];
            cpu.r[3] = cpu.r[3] << 2; // shll2 r3
            cpu.r[4] = cpu.r[15];
            cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
            cpu.r[5] = cpu.r[5].wrapping_add(4);
            cpu.pr = 0x8c080320;
            cpu.r[4] = cpu.r[4].wrapping_add(12); // delay slot: add #12,r4
            crate::registry::p_8c079cac::run(cpu, mem);
        } else {
            // 0x8c080324..0x8c080338: wrapped end, (end-capacity)*4.
            cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(16)).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_sub(cpu.r[1]);
            cpu.r[3] = cpu.r[3] << 2;
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
            cpu.r[0] = STORE_THREE;
            cpu.r[5] = cpu.r[11];
            cpu.r[4] = cpu.r[15];
            cpu.r[7] = cpu.r[3];
            cpu.r[5] = cpu.r[5].wrapping_add(4);
            cpu.pr = 0x8c08033a;
            cpu.r[4] = cpu.r[4].wrapping_add(12); // delay slot: add #12,r4
            crate::registry::p_8c079cac::run(cpu, mem);
        }

        // 0x8c08033a..0x8c080344: copy the end iterator to frame+36.
        cpu.r[0] = COPY_THREE;
        cpu.r[5] = cpu.r[15];
        cpu.r[4] = cpu.r[15];
        cpu.r[5] = cpu.r[5].wrapping_add(12);
        cpu.pr = 0x8c080346;
        cpu.r[4] = cpu.r[4].wrapping_add(36); // delay slot: add #36,r4
        crate::registry::p_8c079c9c::run(cpu, mem);

        // 0x8c080346..0x8c08034c: decrement it (count-1, pointer-4).
        cpu.r[0] = DEC_FIELD;
        cpu.r[4] = cpu.r[15];
        cpu.pr = 0x8c08034e;
        cpu.r[4] = cpu.r[4].wrapping_add(36); // delay slot: add #36,r4
        crate::registry::p_8c079c78::run(cpu, mem);

        // 0x8c08034e..0x8c080362: begin iterator {desc+4, 0, items+start*4}.
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(12)).unwrap_or(0);
        cpu.r[4] = cpu.r[15];
        cpu.r[7] = mem.read_u32(cpu.r[11].wrapping_add(16)).unwrap_or(0);
        cpu.r[0] = cpu.r[0] << 2; // shll2 r0
        cpu.r[14] = mem.read_u32(cpu.r[15].wrapping_add(40)).unwrap_or(0);
        cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[0]);
        cpu.r[11] = cpu.r[11].wrapping_add(4);
        cpu.r[0] = STORE_THREE;
        cpu.r[6] = 0;
        cpu.pr = 0x8c080364;
        cpu.r[5] = cpu.r[11]; // delay slot: mov r11,r5
        crate::registry::p_8c079cac::run(cpu, mem);

        // 0x8c080364..0x8c08036c: copy it to frame+24.
        cpu.r[0] = COPY_THREE;
        cpu.r[4] = cpu.r[15];
        cpu.r[5] = cpu.r[15];
        cpu.pr = 0x8c08036e;
        cpu.r[4] = cpu.r[4].wrapping_add(24); // delay slot: add #24,r4
        crate::registry::p_8c079c9c::run(cpu, mem);

        // 0x8c08036e..0x8c080376: count down from 0 while != count-1.
        cpu.r[12] = mem.read_u32(cpu.r[15].wrapping_add(28)).unwrap_or(0);
        cpu.r[13] = mem.read_u32(cpu.r[15].wrapping_add(24)).unwrap_or(0);
        cpu.set_t(cpu.r[12] == cpu.r[14]); // cmp/eq r14,r12
        cpu.r[11] = mem.read_u32(cpu.r[15].wrapping_add(32)).unwrap_or(0); // bt.s delay slot
        if !cpu.t() {
            loop {
                // 0x8c080378..0x8c08038e: free *cursor, advance, wrap.
                cpu.r[0] = FREE_THUNK;
                cpu.pr = 0x8c08037e;
                cpu.r[4] = mem.read_u32(cpu.r[11]).unwrap_or(0); // delay slot: mov.l @r11,r4
                crate::registry::p_8c01032c::run(cpu, mem);
                cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
                cpu.r[11] = cpu.r[11].wrapping_add(4);
                cpu.r[1] = mem.read_u32(cpu.r[13]).unwrap_or(0);
                cpu.r[0] = cpu.r[3];
                cpu.r[1] = cpu.r[1] << 2; // shll2 r1
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
                cpu.set_t(cpu.r[0] == cpu.r[11]); // cmp/eq r11,r0
                let at_end = cpu.t();
                cpu.r[12] = cpu.r[12].wrapping_add(1); // bf.s delay slot
                if at_end {
                    cpu.r[11] = cpu.r[3]; // wrapped to items
                }
                cpu.set_t(cpu.r[14] == cpu.r[12]); // cmp/eq r14,r12
                if cpu.t() {
                    break;
                }
            }
        }
    }

    // 0x8c080396..0x8c0803a2: pop frame, pr, r11..r14 (last in the rts slot).
    cpu.r[15] = cpu.r[15].wrapping_add(48);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
