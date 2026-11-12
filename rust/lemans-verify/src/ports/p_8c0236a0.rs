//! 0x8c0236a0 — two-pass linear scan over a 0x20-byte descriptor table.
//!
//! `param_1` (r4) points at `{ u32 pad; u32 count; u32 base; }`; `param_2`
//! (r5) is the search key passed to the descriptor comparator 0x8c0209a0.
//!
//! Pass 1 (0x8c0236e2) walks `base .. base + count*0x20` and returns the first
//! record for which the comparator returns nonzero, byte +0x1d is nonzero and
//! byte +0x1c is zero.
//!
//! Pass 2 (0x8c023736) rescans the same window and returns the first record
//! for which the comparator returns nonzero and byte +0x1d is nonzero, when
//! byte +0x1f is zero or byte +0x1c is zero. If the window is exhausted it
//! returns 0. Pass 2's `+0x1c == 0` arm is unreachable from a pure comparator
//! (pass 1 returns such a record first) but is transcribed in full.
//!
//! Side effects: pushes r14/r13/r12/r11/pr on the stack and pops them back;
//! r0 = matched record pointer or 0; r1/r4/r5/r6 as left by the last callee
//! call; T = last tst/cmp.

use sh4_oracle::{Cpu, Memory};

const MATCH: u32 = 0x8c0209a0;

fn ld(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

/// `mov.b @Rn,R0` + `extu.b r0,r0` twice: the byte reaches r0 zero-extended.
fn ldb(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0236a0..0x8c0236ac — prologue
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[11] = cpu.r[4];
    cpu.r[14] = ld(mem, cpu.r[11].wrapping_add(8)); // 0x8c0236ae bra delay slot

    'search: {
        // pass 1 condition and scan — 0x8c0236e2 / 0x8c0236b2
        loop {
            cpu.r[1] = ld(mem, cpu.r[11].wrapping_add(4)); // count
            cpu.r[13] = ld(mem, cpu.r[11].wrapping_add(8)); // base
            cpu.r[1] <<= 2;
            cpu.r[1] <<= 2;
            cpu.r[0] = cpu.r[13];
            cpu.r[1] <<= 1; // count * 0x20
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
            cpu.set_t(cpu.r[0] > cpu.r[14]); // cmp/hi r14,r0
            if !cpu.t() {
                break; // window exhausted -> pass 2
            }
            cpu.r[0] = MATCH;
            cpu.r[5] = cpu.r[14];
            cpu.pr = 0x8c0236ba; // jsr return address
            cpu.r[4] = cpu.r[12]; // jsr delay slot
            crate::registry::p_8c0209a0::run(cpu, mem);
            cpu.r[0] &= 0xff; // extu.b r0,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.t() {
                cpu.r[14] = cpu.r[14].wrapping_add(0x20); // 0x8c0236e0
                continue;
            }
            cpu.r[0] = cpu.r[14].wrapping_add(29); // 0x8c0236c0
            cpu.r[0] = ldb(mem, cpu.r[0]);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.t() {
                cpu.r[14] = cpu.r[14].wrapping_add(0x20);
                continue;
            }
            cpu.r[0] = cpu.r[14].wrapping_add(28); // 0x8c0236ce
            cpu.r[0] = ldb(mem, cpu.r[0]);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if !cpu.t() {
                cpu.r[14] = cpu.r[14].wrapping_add(0x20); // bf 0x8c0236e0
                continue;
            }
            cpu.r[0] = cpu.r[14]; // 0x8c0236dc bra delay slot
            break 'search;
        }

        // pass 2 condition and scan — 0x8c023736 / 0x8c0236f8
        loop {
            cpu.r[0] = ld(mem, cpu.r[11].wrapping_add(4)); // count
            cpu.r[1] = ld(mem, cpu.r[11].wrapping_add(8)); // base
            cpu.r[0] <<= 2;
            cpu.r[0] <<= 2;
            cpu.r[0] <<= 1; // count * 0x20
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
            cpu.set_t(cpu.r[1] > cpu.r[13]); // cmp/hi r13,r1
            if !cpu.t() {
                cpu.r[0] = 0; // 0x8c023746 mov #0,r0
                break 'search;
            }
            cpu.r[0] = MATCH;
            cpu.r[5] = cpu.r[13];
            cpu.pr = 0x8c023700; // jsr return address
            cpu.r[4] = cpu.r[12]; // jsr delay slot
            crate::registry::p_8c0209a0::run(cpu, mem);
            cpu.r[0] &= 0xff; // extu.b r0,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.t() {
                cpu.r[13] = cpu.r[13].wrapping_add(0x20); // 0x8c023734
                continue;
            }
            cpu.r[0] = cpu.r[13].wrapping_add(29); // 0x8c023706
            cpu.r[0] = ldb(mem, cpu.r[0]);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.t() {
                cpu.r[13] = cpu.r[13].wrapping_add(0x20);
                continue;
            }
            cpu.r[0] = cpu.r[13].wrapping_add(31); // 0x8c023714
            cpu.r[0] = ldb(mem, cpu.r[0]);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.t() {
                cpu.r[0] = cpu.r[13]; // 0x8c023730 bra delay slot
                break 'search;
            }
            cpu.r[0] = cpu.r[13].wrapping_add(28); // 0x8c023722
            cpu.r[0] = ldb(mem, cpu.r[0]);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if !cpu.t() {
                cpu.r[13] = cpu.r[13].wrapping_add(0x20); // bf 0x8c023734
                continue;
            }
            cpu.r[0] = cpu.r[13]; // 0x8c023730 bra delay slot
            break 'search;
        }
    }

    // 0x8c023748 — epilogue
    cpu.pr = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = ld(mem, cpu.r[15]); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
