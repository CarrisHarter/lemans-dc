//! 0x8c111ca0 — sort three slots ascending by field-68 key.
//!
//! Slots are `A=[r4]`, `B=[r5]`, `C=[r6]`; each holds an element whose key is
//! `[[elem]+68]` via 0x8c10e35c. Three stages (insertion network):
//!
//! ```text
//! S1: if key(A) > key(B) (signed): swap [r11],[r12]
//! S2: if key(B') <= key(C): done (bf straight to the epilogue)
//!     else: [r13] = B; if key(A) > key(C): [r12] = A, [r11] = C (rotate)
//!                       else:               [r12] = C            (swap B,C)
//! ```
//!
//! `bf` is plain (no delay slot); the `bra` over `[r12]=r14` runs its delay
//! slot `[r11]=r14`. Callee-saved r10-r14/pr/r15 are restored; r0, r1, r4 and
//! T keep the last stage's values. Stack traffic is excluded from the diff, so
//! the spill slots are modelled as locals.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: r11 = r4, r12 = r5, r13 = r6; r10-r14/pr pushed.
    let (save10, save11, save12, save13, save14) =
        (cpu.r[10], cpu.r[11], cpu.r[12], cpu.r[13], cpu.r[14]);
    cpu.r[11] = cpu.r[4];
    cpu.r[12] = cpu.r[5];
    cpu.r[13] = cpu.r[6];

    // Stage 1: order A=[r11] below B=[r12].
    cpu.r[4] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    cpu.r[10] = mem.read_u32(cpu.r[11]).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c10e35c::run(cpu, mem);
    cpu.r[14] = cpu.r[0]; // keyB
    cpu.r[4] = cpu.r[10]; // delay slot of `jsr`
    crate::registry::p_8c10e35c::run(cpu, mem); // r0 = keyA
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32));
    if cpu.t() {
        cpu.r[1] = mem.read_u32(cpu.r[11]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
        let _ = mem.write_u32(cpu.r[12], cpu.r[1]);
    }

    // Stage 2: compare B' against C; taken-`bf` lands on the epilogue.
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.r[10] = mem.read_u32(cpu.r[12]).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c10e35c::run(cpu, mem);
    cpu.r[14] = cpu.r[0]; // keyC
    cpu.r[4] = cpu.r[10]; // delay slot of `jsr`
    crate::registry::p_8c10e35c::run(cpu, mem); // r0 = keyB'
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32));
    if cpu.t() {
        cpu.r[14] = mem.read_u32(cpu.r[13]).unwrap_or(0); // saved C element
        cpu.r[4] = cpu.r[14];
        cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]); // [C] = B
        cpu.r[10] = mem.read_u32(cpu.r[11]).unwrap_or(0); // delay slot of `jsr`
        crate::registry::p_8c10e35c::run(cpu, mem); // r0 = keyC
        cpu.r[13] = cpu.r[0];
        cpu.r[4] = cpu.r[10]; // delay slot of `jsr`
        crate::registry::p_8c10e35c::run(cpu, mem); // r0 = keyA
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[13] as i32));
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[11]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[12], cpu.r[0]); // [B] = A
            let _ = mem.write_u32(cpu.r[11], cpu.r[14]); // delay slot of `bra`
        } else {
            let _ = mem.write_u32(cpu.r[12], cpu.r[14]); // [B] = C
        }
    }

    // Epilogue: restore r10-r14 (pr/r15 balanced).
    cpu.r[10] = save10;
    cpu.r[11] = save11;
    cpu.r[12] = save12;
    cpu.r[13] = save13;
    cpu.r[14] = save14;
}
