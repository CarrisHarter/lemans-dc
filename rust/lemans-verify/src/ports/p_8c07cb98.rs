//! 0x8c07cb98 — dispatch on op (r5) over a float struct at r4, then clamp
//! *(float*)(r4+0x1c) into [0,1].
//!
//! CFG sketch:
//!   op == 5 → return *(u32*)(r4+0x28)          (rts delay slot)
//!   op == 6 → return *(u32*)(r4+0x2c)
//!   fr1 = *(f32*)(r4+0x10), fr0 = *(f32*)(r4+0x14); gt = fr1 > fr0
//!   gt:  op 1 → return *(u32*)(r4+0x20); op 2 → return *(u32*)(r4+0x24);
//!        op 3 → *(f32*)(r4+0x1c) = *(r4+0x1c) - *(r4+0x18); op 4 → += ;
//!        other → nothing
//!   !gt: op 1 → subtract-store; op 2 → add-store;
//!        op 3 → return *(u32*)(r4+0x20); op 4 → return *(u32*)(r4+0x24);
//!        other → nothing
//!   all compute paths: r4 += 0x1c, then clamp v = *(f32*)(r4+0x1c):
//!        v < 0 → store 0.0 (T set); v > 1 → store 1.0 (T set); else no store
//!   return 0 (rts delay slot mov #0,r0)
//!
//! Side effects the decompiled C does not show: on compute paths r4 ends at
//! base+0x1c (clobbered), r3 := base+0x10, r1 := base+0x14 on the plain path
//! but base+0x1c on the store paths, r0 := 0 via the rts delay slot, fr0 ends
//! as 0.0/1.0 from the fldi and fr1 as the clamped-source value, and T is set
//! exactly when a clamp store happened (or on the early-return dispatch, where
//! it is the matched cmp/eq #n). Early returns leave r1/r3/r4/fr at seeds.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let op = cpu.r[5];
    cpu.r[0] = op; // mov r5,r0
    cpu.set_t(op == 5); // cmp/eq #5,r0
    if op == 5 {
        cpu.r[0] = mem.read_u32(base.wrapping_add(40)).unwrap_or(0); // rts delay slot
        return;
    }
    cpu.set_t(op == 6); // cmp/eq #6,r0
    if op == 6 {
        cpu.r[0] = mem.read_u32(base.wrapping_add(44)).unwrap_or(0); // rts delay slot
        return;
    }
    // 0x8c07cbae
    cpu.r[3] = base.wrapping_add(16);
    cpu.r[1] = base.wrapping_add(20);
    let fr1 = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr1
    let fr0 = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0
    cpu.fr[1] = fr1;
    cpu.fr[0] = fr0;
    cpu.set_t(fr1 > fr0); // fcmp/gt fr0,fr1 → T = fr1 > fr0
    let gt = fr1 > fr0;
    let mut store_add: Option<bool> = None; // Some(true)=fadd, Some(false)=fsub
    if gt {
        cpu.set_t(op == 1);
        if op == 1 {
            cpu.r[0] = mem.read_u32(base.wrapping_add(32)).unwrap_or(0); // rts delay
            return;
        }
        cpu.set_t(op == 2);
        if op == 2 {
            cpu.r[0] = mem.read_u32(base.wrapping_add(36)).unwrap_or(0);
            return;
        }
        cpu.set_t(op == 3);
        if op == 3 {
            store_add = Some(false); // fsub at 0x8c07cbda
        } else {
            cpu.set_t(op == 4);
            if op == 4 {
                store_add = Some(true); // fadd at 0x8c07cbec
            }
        }
    } else {
        cpu.set_t(op == 1);
        if op == 1 {
            store_add = Some(false); // fsub at 0x8c07cc12
        } else {
            cpu.set_t(op == 2);
            if op == 2 {
                store_add = Some(true); // fadd at 0x8c07cc24
            } else {
                cpu.set_t(op == 3);
                if op == 3 {
                    cpu.r[0] = mem.read_u32(base.wrapping_add(32)).unwrap_or(0);
                    return;
                }
                cpu.set_t(op == 4);
                if op == 4 {
                    cpu.r[0] = mem.read_u32(base.wrapping_add(36)).unwrap_or(0);
                    return;
                }
            }
        }
    }
    if let Some(is_add) = store_add {
        let r1 = base.wrapping_add(28);
        let r0 = base.wrapping_add(24);
        let a = mem.read_f32(r1).unwrap_or(0.0);
        let b = mem.read_f32(r0).unwrap_or(0.0);
        cpu.fr[1] = a;
        cpu.fr[0] = b;
        cpu.fr[1] = if is_add { a + b } else { a - b }; // fadd/fsub fr0,fr1
        let _ = mem.write_f32(r1, cpu.fr[1]); // delay slot of bra 0x8c07cc3e
        cpu.r[1] = r1;
    }
    // 0x8c07cc3e: clamp *(f32*)(r4+0x1c) into [0,1]
    cpu.r[4] = base.wrapping_add(28);
    let v = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // fmov @r4,fr1
    cpu.fr[1] = v;
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.set_t(0.0 > v); // fcmp/gt fr1,fr0 → T = fr0 > fr1
    if 0.0 > v {
        let _ = mem.write_f32(cpu.r[4], 0.0); // delay slot of bra 0x8c07cc54
        cpu.r[0] = 0; // rts delay slot mov #0,r0
        return;
    }
    cpu.fr[0] = 1.0; // fldi1 fr0
    cpu.set_t(v > 1.0); // fcmp/gt fr0,fr1 → T = fr1 > fr0
    if v > 1.0 {
        let _ = mem.write_f32(cpu.r[4], 1.0); // 0x8c07cc52
    }
    cpu.r[0] = 0; // rts delay slot mov #0,r0
}
