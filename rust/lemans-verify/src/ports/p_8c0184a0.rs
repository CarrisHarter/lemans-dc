//! 0x8c0184a0 — pick whichever of two candidate descriptors carries the tag.
//!
//! The whole routine is a guarded two-way lookup:
//!
//! ```text
//! mov.l 0x8c0184c4,r0 ; mov.l @r0,r0 ; stc sr,r1 ; tst r0,r1 ; bt 0x8c0184f8
//! ```
//!
//! The `tst` masks SR with the dword stored at 0x8c018508 (`0x40000000`, the MD
//! bit). When MD is clear the routine returns the default pointer found through
//! 0x8c018504 (`0x8c018498`) and leaves r2/r3 alone.
//!
//! Otherwise it compares two 6-byte tags `{u32, s16}`: a candidate pointer from
//! 0x8c01850c, then one from 0x8c018510, against the reference dword at
//! 0x8c018514 (`0x000b003b`) and sign-extended word at 0x8c018518 (`0x0009`).
//! The matching candidate pointer is returned in r0, else 0. Both `mov.w`
//! loads sign-extend, and r2 ends post-incremented to 0x8c018518 on every path
//! that reaches a comparison. The T residue is the last `cmp/eq`: set when a
//! candidate matched, clear when both failed.
//!
//! The literal-pool entries and the tag bytes are read out of memory rather
//! than baked in, so a spec that redirects the candidates or retags them is
//! followed exactly like the original.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0184c4).unwrap_or(0); // mov.l @(0x8c0184c4,pc),r0 -> 0x8c018508
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r0 -> SR mask 0x40000000
    cpu.r[1] = cpu.sr; // stc sr,r1
    cpu.set_t(cpu.r[0] & cpu.r[1] == 0); // tst r0,r1
    if cpu.t() {
        // bt 0x8c0184f8: SR.MD clear, return the default descriptor.
        cpu.r[0] = mem.read_u32(0x8c018500).unwrap_or(0); // mov.l @(0x8c018500,pc),r0 -> 0x8c018504
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // rts delay: mov.l @r0,r0 -> 0x8c018498
        return; // rts -> pr
    }

    cpu.r[0] = mem.read_u32(0x8c0184c8).unwrap_or(0); // mov.l @(0x8c0184c8,pc),r0 -> 0x8c01850c
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r3 -> first candidate
    cpu.r[2] = mem.read_u32(0x8c0184cc).unwrap_or(0); // mov.l @(0x8c0184cc,pc),r2 -> 0x8c018514
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r0 (tag dword)
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2+,r1 (reference dword)
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
    if cpu.t() {
        cpu.r[0] =
            mem.read_u16(cpu.r[3].wrapping_add(4)).unwrap_or(0) as i16 as i32 as u32; // mov.w @(4,r3),r0
        cpu.r[1] = mem.read_u16(cpu.r[2]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r2,r1
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
        if cpu.t() {
            cpu.r[0] = cpu.r[3]; // rts delay: mov r3,r0
            return; // rts -> pr
        }
    }

    // bf 0x8c0184d0 — the first candidate does not carry the tag.
    cpu.r[0] = mem.read_u32(0x8c0184ec).unwrap_or(0); // mov.l @(0x8c0184ec,pc),r0 -> 0x8c018510
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r3 -> second candidate
    cpu.r[2] = mem.read_u32(0x8c0184f0).unwrap_or(0); // mov.l @(0x8c0184f0,pc),r2 -> 0x8c018514
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r0 (tag dword)
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2+,r1 (reference dword)
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
    if cpu.t() {
        cpu.r[0] =
            mem.read_u16(cpu.r[3].wrapping_add(4)).unwrap_or(0) as i16 as i32 as u32; // mov.w @(4,r3),r0
        cpu.r[1] = mem.read_u16(cpu.r[2]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r2,r1
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
        if cpu.t() {
            cpu.r[0] = cpu.r[3]; // rts delay: mov r3,r0
            return; // rts -> pr
        }
    }

    cpu.r[0] = 0; // 0x8c0184f4 rts; delay slot: mov #0,r0
}
