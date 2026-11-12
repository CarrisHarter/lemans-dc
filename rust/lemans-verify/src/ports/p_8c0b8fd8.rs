//! 0x8c0b8fd8 — walk a path string against a directory/leaf node list.
//!
//! r4 points at a header whose +4 word is the head of a node list, r5 is the
//! path to resolve, and the caller's first stack word — fetched here with the
//! `mov.l @(r0,r15)` indexed load, r0 = 0x214 — points at the 8-byte result.
//! A node is 12 bytes plus a NUL-terminated name: `next @0`, `kind @4`,
//! `payload @8`, name @12. `kind == -1` marks a directory and `payload` is then
//! its child list; any other kind is a leaf and both words are reported.
//!
//! The routine refills a 512-byte frame one path component at a time (component
//! copy at +0, working path at +0x100), searches the current list with a byte
//! compare, then either reports the node (kind != -1 and the path is exhausted),
//! descends into `node->payload` when kind == -1, or re-walks the same list when
//! the path still has components. Both "nothing matched" exits clear the two
//! result words. Side effects the C hides: the epilogue always leaves
//! r2 = 0x200; r12 is reused as the -1 for the kind test after the entry header
//! is consumed; r13 ends up holding `path + i` (a pointer, not the index) on the
//! straight-line exits; r11..r14 are restored from the frame.
//!
//! Callees are the already ported routines; every jsr's delay slot feeds the
//! argument register before the call and pr gets the original return address.

use sh4_oracle::{Cpu, Memory};

use super::{p_8c0112f4, p_8c011344, p_8c017f48};

/// 0x8c0b90d0 — frame offset of the caller's stack argument.
const LIT_D0: u32 = 0x214;
/// 0x8c0b90d4 — frame offset of the working path copy.
const LIT_D4: u32 = 0x100;
/// 0x8c0b90d8 — strcpy.
const LIT_D8: u32 = 0x8c017f48;
/// 0x8c0b90dc — strlen.
const LIT_DC: u32 = 0x8c0112f4;
/// 0x8c0b90e0 — strncpy.
const LIT_E0: u32 = 0x8c011344;
/// 0x8c0b90e4 — frame plus saved registers, restored by the epilogue.
const LIT_E4: u32 = 0x200;

/// `mov.b` / `exts.b`: the oracle sign-extends every byte load.
#[inline]
fn sext8(v: u8) -> u32 {
    v as i8 as i32 as u32
}

/// 0x8c0b90c0 — drop the frame and pop the saved registers (r15 ends at the
/// caller's value, r2 always ends 0x200).
fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = LIT_E4; // mov.l 0x8c0b90e4,r2
    cpu.r[15] = cpu.r[15].wrapping_add(cpu.r[2]); // add r2,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [11usize, 12, 13, 14] {
        cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,rn
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    // rts / delay slot `mov.l @r15+,r14` are covered by the loop above.
}

/// 0x8c0b9040 and 0x8c0b90ba: clear both result words and fall into the
/// epilogue. The `mov.l r0,@(4,r11)` runs in the `bra` delay slot.
fn report_none(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[11], 0); // mov.l r0,@r11
    let _ = mem.write_u32(cpu.r[11].wrapping_add(4), 0); // delay slot
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[15];
    // 0x8c0b8fd8: push r14..r11, r12 = parameter, push pr, then -128 x4.
    for (i, reg) in [14usize, 13, 12, 11].into_iter().enumerate() {
        let _ = mem.write_u32(s.wrapping_sub(4 * (i as u32 + 1)), cpu.r[reg]);
    }
    cpu.r[12] = cpu.r[4]; // 0x8c0b8fe0: mov r4,r12
    let _ = mem.write_u32(s.wrapping_sub(20), cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = s.wrapping_sub(532); // add #-128,r15 x4

    // 0x8c0b8fea: mov.l @(r0,r15),r11 — the result pointer lives in the
    // caller's stack argument at the original r15.
    cpu.r[0] = LIT_D0;
    cpu.r[11] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0);
    cpu.r[4] = LIT_D4; // 0x8c0b8ff0
    cpu.r[0] = LIT_D8; // 0x8c0b8ff2
    cpu.pr = 0x8c0b8ff8; // 0x8c0b8ff4: jsr @r0
    cpu.r[4] = cpu.r[15].wrapping_add(cpu.r[4]); // delay: add r15,r4
    p_8c017f48::run(cpu, mem); // strcpy(path, r5)
    cpu.r[14] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0); // 0x8c0b8ff8
    cpu.r[12] = u32::MAX; // 0x8c0b8ffc: mov #-1,r12

    'walk: loop {
        // ---- 0x8c0b9064: cut the first component out of the working path ----
        cpu.r[4] = LIT_D4;
        cpu.r[0] = LIT_DC;
        cpu.r[4] = cpu.r[15].wrapping_add(cpu.r[4]); // delay: add r15,r4
        cpu.pr = 0x8c0b906c;
        p_8c0112f4::run(cpu, mem); // strlen(path)
        cpu.r[1] = LIT_D4; // 0x8c0b906c
        cpu.r[3] = cpu.r[0]; // 0x8c0b906e: mov r0,r3
        cpu.r[13] = 0; // 0x8c0b9070
        cpu.r[1] = cpu.r[15].wrapping_add(cpu.r[1]); // 0x8c0b9074: delay
        loop {
            // 0x8c0b9078: cmp/gt r13,r3 — scan while i < len and no '/'.
            cpu.set_t((cpu.r[3] as i32) > (cpu.r[13] as i32));
            if !cpu.t() {
                break;
            }
            cpu.r[0] = cpu.r[1]; // 0x8c0b907c: mov r1,r0
            // 0x8c0b907e/0x8c0b9080: mov.b @(r0,r13),r0 ; exts.b r0,r0
            cpu.r[0] = sext8(mem.read_u8(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0));
            cpu.set_t(cpu.r[0] == 47); // 0x8c0b9082: cmp/eq #47,r0
            if cpu.t() {
                break;
            }
            cpu.r[13] = cpu.r[13].wrapping_add(1); // 0x8c0b9076
        }
        // 0x8c0b9086: strncpy(base, path, i) — never NUL-terminated by itself.
        cpu.r[5] = LIT_D4;
        cpu.r[6] = cpu.r[13];
        cpu.r[0] = LIT_E0;
        cpu.r[5] = cpu.r[15].wrapping_add(cpu.r[5]); // 0x8c0b908c
        cpu.r[4] = cpu.r[15]; // 0x8c0b9090: delay: mov r15,r4
        cpu.pr = 0x8c0b9090;
        p_8c011344::run(cpu, mem);
        cpu.r[0] = cpu.r[15]; // 0x8c0b9092: mov r15,r0
        cpu.r[1] = 0; // 0x8c0b9094: mov #0,r1
        let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[13]), 0); // 0x8c0b9096
        cpu.r[0] = LIT_D4; // 0x8c0b9098
        cpu.r[0] = cpu.r[15].wrapping_add(cpu.r[0]); // 0x8c0b909a: add r15,r0
        // 0x8c0b909c/0x8c0b909e: the byte that followed the component.
        cpu.r[0] = sext8(mem.read_u8(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0));
        cpu.set_t(cpu.r[0] == 47); // 0x8c0b90a0: cmp/eq #47,r0
        if cpu.t() {
            cpu.r[13] = cpu.r[13].wrapping_add(1); // 0x8c0b90a4: eat the '/'
        }
        // 0x8c0b90a6: strcpy(path, path + i) — r13 becomes the source pointer.
        cpu.r[4] = LIT_D4;
        cpu.r[0] = LIT_D8;
        cpu.r[4] = cpu.r[15].wrapping_add(cpu.r[4]); // 0x8c0b90aa: add r15,r4
        cpu.r[13] = cpu.r[4].wrapping_add(cpu.r[13]); // 0x8c0b90ac: add r4,r13
        cpu.r[5] = cpu.r[13]; // 0x8c0b90b0: delay: mov r13,r5
        cpu.pr = 0x8c0b90b0;
        p_8c017f48::run(cpu, mem);
        // 0x8c0b90b2: an empty component means the whole lookup failed.
        cpu.r[0] = sext8(mem.read_u8(cpu.r[15]).unwrap_or(0));
        cpu.set_t(cpu.r[0] == 0); // 0x8c0b90b6: tst r0,r0
        if cpu.t() {
            report_none(cpu, mem);
            epilogue(cpu, mem);
            return;
        }

        // ---- 0x8c0b8ffe: search the current list for that component ----
        if cpu.r[14] != 0 {
            cpu.r[3] = cpu.r[15]; // 0x8c0b9002: mov r15,r3
            cpu.r[1] = 0; // 0x8c0b9004: mov #0,r1 (the "equal" diff value)
            'list: loop {
                cpu.r[7] = cpu.r[14].wrapping_add(12); // 0x8c0b9006 + delay add
                cpu.r[6] = cpu.r[3]; // 0x8c0b9008: mov r3,r6
                loop {
                    // 0x8c0b9018: compare the component with node+12 byte by
                    // byte; both loads are sign-extended and then zero-extended.
                    cpu.r[5] = sext8(mem.read_u8(cpu.r[6]).unwrap_or(0));
                    cpu.r[0] = sext8(mem.read_u8(cpu.r[7]).unwrap_or(0));
                    cpu.r[5] &= 0xff; // 0x8c0b901c: extu.b r5,r5
                    cpu.r[4] = cpu.r[5] & 0xff; // 0x8c0b901e: extu.b r5,r4
                    cpu.r[0] &= 0xff; // 0x8c0b9020
                    cpu.r[0] &= 0xff; // 0x8c0b9022
                    cpu.r[6] = cpu.r[6].wrapping_add(1); // 0x8c0b9024
                    cpu.set_t(cpu.r[4] == cpu.r[0]); // 0x8c0b9026: cmp/eq r0,r4
                    cpu.r[7] = cpu.r[7].wrapping_add(1); // 0x8c0b902a: bt.s delay
                    if !cpu.t() {
                        cpu.r[4] = cpu.r[4].wrapping_sub(cpu.r[0]); // 0x8c0b902c
                        break;
                    }
                    // 0x8c0b900e: bytes equal — did the component end here?
                    cpu.r[0] = cpu.r[5] & 0xff; // extu.b r5,r0
                    cpu.set_t(cpu.r[0] == 0); // 0x8c0b9010: tst r0,r0
                    if cpu.t() {
                        cpu.r[4] = cpu.r[1]; // 0x8c0b9016: bra delay
                        break;
                    }
                }
                cpu.set_t(cpu.r[4] == 0); // 0x8c0b902e: tst r4,r4
                if !cpu.t() {
                    cpu.r[14] = mem.read_u32(cpu.r[14]).unwrap_or(0); // 0x8c0b9036
                    cpu.set_t(cpu.r[14] == 0); // 0x8c0b9038
                    if !cpu.t() {
                        continue 'list; // 0x8c0b903a: next node, r3/r1 unchanged
                    }
                }
                break; // match, or the list ran out
            }
        }

        // ---- 0x8c0b903c: a match (or the end of the list) ----
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14
        if cpu.t() {
            report_none(cpu, mem);
            epilogue(cpu, mem);
            return;
        }
        cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // 0x8c0b9048
        cpu.set_t(cpu.r[1] == cpu.r[12]); // 0x8c0b904a: cmp/eq r12,r1
        if cpu.t() {
            // 0x8c0b904e: a directory — walk its child list.
            cpu.r[14] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0);
            continue 'walk;
        }
        // 0x8c0b9052: a leaf, but is the path used up?
        cpu.r[0] = LIT_D4;
        cpu.r[0] = sext8(mem.read_u8(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0));
        cpu.set_t(cpu.r[0] == 0); // 0x8c0b9058: tst r0,r0
        if !cpu.t() {
            continue 'walk; // 0x8c0b905a: still components left, same list
        }
        // 0x8c0b905c: report the leaf's payload and kind.
        cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
        let _ = mem.write_u32(cpu.r[11].wrapping_add(4), cpu.r[1]); // bra delay slot
        epilogue(cpu, mem);
        return;
    }
}
