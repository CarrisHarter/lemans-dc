//! 0x8c1732b8 — guarded reporter: call 0x8c1725fc(0x12f, fixed string) or return a global.
//!
//! ```text
//! r1=[pool] (0x8c20d450); push pr; r2=[r1]; tst r2,r2
//! bf Lflag_set                    // plain bf: no delay slot
//! r3=[pool] (0x8c1725fc); r4=[pool].w (0x12f, sign-extended)
//! r5=[pool] (0x8c1df650); jsr @r3; delay nop
//! pop pr; rts; delay mov #0,r0    // call path returns 0
//! Lflag_set: r2=[pool] (0x8c2b526c); r0=[r2]; pop pr; rts; delay nop
//! ```
//!
//! When the flag at 0x8c20d450 is zero the routine reports through the callee
//! (which stores r4=0x12f at 0x8c2b5260+0x2b8 and copies the 54-byte string
//! into 0x8c2b5260+0x1b8 = 0x8c2b5418) and returns 0. When the flag is set it
//! returns the word at 0x8c2b526c untouched.
//!
//! Residue: r15 and pr restored on both paths; r0=0 on the call path,
//! r0=[0x8c2b526c] on the flag-set path; T is the callee's exit flag (call
//! path) or the `tst r2,r2` result — clear — on the flag-set path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8c173374).unwrap_or(0x8c20d450);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.set_t(cpu.r[2] == 0); // tst r2,r2
    if !cpu.t() {
        // Flag set: r0 = [0x8c2b526c].
        cpu.r[2] = mem.read_u32(0x8c173380).unwrap_or(0x8c2b526c);
        cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    cpu.r[3] = mem.read_u32(0x8c17337c).unwrap_or(0x8c1725fc);
    cpu.r[4] = mem.read_u16(0x8c17336e).unwrap_or(0x12f) as i16 as i32 as u32;
    cpu.r[5] = mem.read_u32(0x8c173378).unwrap_or(0x8c1df650);
    cpu.pr = 0x8c1732cc; // jsr @r3; delay nop
    crate::registry::p_8c1725fc::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0; // delay slot of rts: mov #0,r0
}
