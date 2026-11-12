//! 0x8c1044f4 — search a node's child list for the first accepted entry.
//!
//! ```text
//! if (v == 0) return 0;
//! if (v->field14 != 0) return v;
//! for (n = v->field0c; n != 0; n = n->field08) {
//!     r = FUN_8c1044f4(a, n);      // jsr through the literal pool: self
//!     if (r != 0) return r;
//! }
//! return 0;
//! ```
//!
//! `jsr @r0` loads 0x8c1044f4 from the pool at 0x8c104540, so the callee is
//! this same routine: the recursion is ported by calling the registered port,
//! which saves and restores `r12`/`r13`/`r14`/`pr` exactly as the original
//! frame does. The residue of the last call in `r1`, `r4`, `r5` and the T flag
//! survives to the caller, so the body tracks every instruction rather than
//! reducing the routine to its return value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if !cpu.t() {
        // bf 0x8c104506
        cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            // bra 0x8c104534 ; mov r5,r0
            cpu.r[0] = cpu.r[5];
            epilogue(cpu, mem);
            return;
        }

        // 0x8c104510: mov.l @(12,r5),r14 ; mov #0,r0 ; bra 0x8c10452a
        cpu.r[14] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
        cpu.r[0] = 0;
        cpu.r[13] = cpu.r[0]; // delay slot: mov r0,r13

        loop {
            cpu.set_t(cpu.r[14] == 0); // 0x8c10452a: tst r14,r14
            if cpu.t() {
                break; // bt 0x8c104534
            }
            cpu.r[1] = cpu.r[13] & 0xff; // 0x8c10452e: extu.b r13,r1
            cpu.set_t(cpu.r[1] == 0); // 0x8c104530: tst r1,r1
            if !cpu.t() {
                break; // bt 0x8c104518 not taken
            }

            // 0x8c104518: mov.l 0x8c104540,r0 ; mov r14,r5 ; jsr @r0 ;
            // mov r12,r4 (delay slot)
            cpu.r[0] = mem.read_u32(0x8c104540).unwrap_or(0x8c1044f4);
            cpu.r[5] = cpu.r[14];
            cpu.pr = 0x8c104520;
            cpu.r[4] = cpu.r[12];
            crate::registry::p_8c1044f4::run(cpu, mem);

            cpu.set_t(cpu.r[0] == 0); // 0x8c104520: tst r0,r0
            if cpu.t() {
                // bra 0x8c10452a ; mov.l @(8,r14),r14 (delay slot)
                cpu.r[14] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0);
            } else {
                cpu.r[13] = 1; // 0x8c104528: mov #1,r13
            }
        }
    } else {
        cpu.r[0] = 0; // bra 0x8c104534 ; mov #0,r0 (delay slot)
    }

    epilogue(cpu, mem);
}

/// 0x8c104534: `lds.l @r15+,pr ; mov.l @r15+,r12 ; mov.l @r15+,r13 ; rts` with
/// the `mov.l @r15+,r14` delay slot.
fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
