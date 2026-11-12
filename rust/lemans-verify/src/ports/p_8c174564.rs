//! 0x8c174564 — store r4 to a global slot, then copy a string into a fixed buffer.
//!
//! ```text
//! push r14,r13,r12,pr; r13=r5; tst r13,r13
//! r0=[0x8c1745da].w (0x104); r14=[0x8c1745e0] (0x8c2b5528)
//! bf.s L57e; delay: [r14+r0]=r4   // store runs on BOTH paths
//! r1=[0x8c1745e4] (0x8c1e0348); bra L58c   // r5==0 side
//! L57e: r12=[0x8c1745dc].w (0xff); len=strlen(r13); cmp/hs r12,r0
//! bt L598 (plain: no delay slot); not-taken side runs mov r13,r1
//! L58c: strcpy(dst=r14+4, src=r1); bra L5ae
//! L598: strncpy(dst=r14+4, src=r13, n=0xff); [dst+0xff]=0
//! L5ae: pop pr,r12,r13; rts; delay pop r14
//! ```
//!
//! The `bf.s` tests the `tst r13,r13` result: r5!=0 branches to the strlen
//! probe, r5==0 falls through with the fixed "Internal Error" string as the
//! strcpy source. Short strings (len < 0xff) take the strcpy path with
//! src=r13; long ones are truncated via strncpy with an explicit NUL at
//! dst+0xff. The callees run as the registered ports, so their full
//! register/memory effects (including strcpy's alignment-dependent T) come
//! along verbatim.
//!
//! Residue: r0=dst on every path; r12-r15 and pr restored; T is the callee's
//! exit flag (strcpy on L58c, strncpy's set T on L598).

use sh4_oracle::{Cpu, Memory};

/// L58c: strcpy(dst=r14+4, src=r1). Reached from the r5==0 fallthrough
/// (src = fixed string) and from the short-strlen side (src = r13).
fn l58c_strcpy(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c1745ec).unwrap_or(0x8c016484);
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = 0x8c174594; // jsr @r2
    cpu.r[0] = cpu.r[0].wrapping_add(4); // delay slot: add #4,r0
    crate::registry::p_8c016484::run(cpu, mem);
    // bra L5ae; delay nop
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // push r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // push r13
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]); // push r12
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[0] = mem.read_u16(0x8c1745da).unwrap_or(0) as i16 as i32 as u32; // 0x104
    cpu.r[14] = mem.read_u32(0x8c1745e0).unwrap_or(0x8c2b5528);
    // bf.s L57e — the delay-slot store runs whether it branches or not.
    let to_l57e = !cpu.t();
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[4]); // mov.l r4,@(r0,r14)
    if to_l57e {
        // L57e: probe the string length.
        cpu.r[2] = mem.read_u32(0x8c1745e8).unwrap_or(0x8c0112f4);
        cpu.r[12] = mem.read_u16(0x8c1745dc).unwrap_or(0) as i16 as i32 as u32; // 0xff
        cpu.pr = 0x8c174586; // jsr @r2
        cpu.r[4] = cpu.r[13]; // delay slot: mov r13,r4
        crate::registry::p_8c0112f4::run(cpu, mem);
        cpu.set_t(cpu.r[0] >= cpu.r[12]); // cmp/hs r12,r0
        if cpu.t() {
            // L598: long string — bounded copy plus an explicit NUL.
            cpu.r[4] = cpu.r[14]; // mov r14,r4
            cpu.r[6] = cpu.r[12]; // mov r12,r6 (n = 0xff)
            cpu.r[2] = mem.read_u32(0x8c1745f0).unwrap_or(0x8c011344);
            cpu.r[5] = cpu.r[13]; // mov r13,r5
            cpu.pr = 0x8c1745a4; // jsr @r2
            cpu.r[4] = cpu.r[4].wrapping_add(4); // delay slot: add #4,r4
            crate::registry::p_8c011344::run(cpu, mem);
            cpu.r[3] = cpu.r[14]; // mov r14,r3
            cpu.r[3] = cpu.r[3].wrapping_add(4); // r3 = dst
            cpu.r[2] = 0; // mov #0,r2
            cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[3]); // r12 = dst + 0xff
            let _ = mem.write_u8(cpu.r[12], cpu.r[2] as u8); // mov.b r2,@r12
        } else {
            cpu.r[1] = cpu.r[13]; // mov r13,r1 (plain bt: skipped when taken)
            l58c_strcpy(cpu, mem);
        }
    } else {
        cpu.r[1] = mem.read_u32(0x8c1745e4).unwrap_or(0x8c1e0348);
        // bra L58c; delay nop
        l58c_strcpy(cpu, mem);
    }
    // L5ae: pop pr,r12,r13; rts; delay pop r14.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
