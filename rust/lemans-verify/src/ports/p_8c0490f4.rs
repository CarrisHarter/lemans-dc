//! 0x8c0490f4 — per-object state machine tick around 0x8c0326a0/0x8c04b360.
//!
//! Field layout on the object in `r4`: +0x2c pointer to a struct whose +0xc
//! pointer leads to a flag word (bit 0x100), +0x190 pointer passed to
//! 0x8c0326a0, +0x288/+0x28c/+0x290/+0x294 float fields, +0x298/+0x299/+0x29a
//! one-byte state flags. The globals at 0x8c23dab0 and 0x8c2ab460 hold object
//! pointers; their +0x48/+0x4c reach counters used to pick a transition.
//!
//! Both indirect calls resolve to fixed pool entries: 0x8c0326a0 (called with
//! `r4` object pointer and `fr4` float) and 0x8c04b360 (`r4` pointer, `r5` = 0).
//! Hidden side effects: five literal-pool loads whose values land in r0/r1/r4,
//! the 0x298/0x299/0x29a byte transitions, the +0x290 float accumulation, and
//! the stack frame holding fr15/r14/pr.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // fmov fr15,@-r15 ; mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4];
    let base = cpu.r[14];
    cpu.r[1] = mem.read_u32(0x8C04923C).unwrap(); // 0x298
    cpu.r[3] = base;
    cpu.r[4] = mem.read_u32(0x8C049240).unwrap(); // 0x294
    cpu.r[0] = cpu.r[1];
    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(base)).unwrap() as i8 as i32 as u32;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.fr[15] = mem.read_f32(cpu.r[3]).unwrap(); // delay slot of `bf.s`

    if cpu.t() {
        path_a(cpu, mem, base); // 0x8c049112
    } else {
        path_b(cpu, mem, base); // 0x8c049160
    }

    // lds.l @r15+,pr ; mov.l @r15+,r14 ; rts ; fmov @r15+,fr15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

/// 0x8c049112 — the object had never entered this state.
fn path_a(cpu: &mut Cpu, mem: &mut Memory, base: u32) {
    cpu.r[0] = mem.read_u32(0x8C049244).unwrap(); // 0x8c23dab0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap();
    cpu.r[0] = cpu.r[0].wrapping_add(72);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(36)).unwrap();
    cpu.set_t(cpu.r[0] == 10); // cmp/eq #10,r0
    if cpu.t() {
        // 0x8c049120: enter the 10-count state, latch fr15 into +0x290.
        cpu.r[0] = cpu.r[1]; // 0x298
        cpu.r[1] = 1;
        let _ = mem.write_u8(cpu.r[0].wrapping_add(base), cpu.r[1] as u8);
        cpu.r[3] = base;
        cpu.r[0] = mem.read_u32(0x8C049248).unwrap(); // 0x299
        let _ = mem.write_u8(cpu.r[0].wrapping_add(base), cpu.r[1] as u8);
        cpu.r[0] = mem.read_u32(0x8C04924C).unwrap(); // 0x290
        cpu.r[1] = base;
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
        cpu.r[0] = mem.read_u32(0x8C049250).unwrap(); // 0x28c
        let _ = mem.write_f32(cpu.r[3], cpu.fr[15]);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        cpu.r[0] = mem.read_u32(0x8C049254).unwrap(); // 0x190
        cpu.fr[4] = mem.read_f32(cpu.r[1]).unwrap();
        cpu.r[1] = mem.read_u32(0x8C049258).unwrap(); // 0x8c0326a0
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(base)).unwrap();
        cpu.pr = 0x8C049142;
        crate::registry::p_8c0326a0::run(cpu, mem);
        // bra 0x8c049234
    } else {
        // 0x8c049146: enter the state with the +0x288 float.
        cpu.r[0] = cpu.r[1]; // 0x298
        cpu.r[1] = 1;
        let _ = mem.write_u8(cpu.r[0].wrapping_add(base), cpu.r[1] as u8);
        cpu.r[0] = mem.read_u32(0x8C04925C).unwrap(); // 0x288
        cpu.r[1] = base;
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        cpu.r[0] = mem.read_u32(0x8C049254).unwrap(); // 0x190
        cpu.fr[4] = mem.read_f32(cpu.r[1]).unwrap();
        cpu.r[1] = mem.read_u32(0x8C049258).unwrap(); // 0x8c0326a0
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(base)).unwrap();
        cpu.pr = 0x8C04915C;
        crate::registry::p_8c0326a0::run(cpu, mem);
        // bra 0x8c049234
    }
}

/// 0x8c049160 — the object is already past the first state.
fn path_b(cpu: &mut Cpu, mem: &mut Memory, base: u32) {
    cpu.r[0] = mem.read_u32(0x8C049248).unwrap(); // 0x299
    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(base)).unwrap() as i8 as i32 as u32;
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        path_b2(cpu, mem, base); // 0x8c0491c0
        return;
    }

    // 0x8c04916c: second state, look at the global object's +0x48 pointer.
    cpu.r[0] = mem.read_u32(0x8C049244).unwrap(); // 0x8c23dab0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap();
    cpu.r[0] = cpu.r[0].wrapping_add(0x48);
    cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap();
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.t() {
        return; // bt 0x8c049234
    }

    cpu.r[0] = mem.read_u32(0x8C049260).unwrap(); // 0x8c04b360
    cpu.r[5] = 0; // delay slot of `jsr`
    cpu.pr = 0x8C04917E;
    crate::registry::p_8c04b360::run(cpu, mem);
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        // 0x8c049184: add the global's +0xa4 float to our +0x290.
        cpu.r[0] = mem.read_u32(0x8C049244).unwrap(); // 0x8c23dab0
        cpu.r[3] = base;
        cpu.r[4] = mem.read_u32(0x8C04924C).unwrap(); // 0x290
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
        cpu.r[0] = mem.read_u32(0x8C049264).unwrap(); // 0xa4
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
        let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    }

    // 0x8c04919a: fire once the accumulated value passes fr15.
    cpu.r[1] = mem.read_u32(0x8C04924C).unwrap(); // 0x290
    cpu.r[0] = base;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.set_t(cpu.fr[0] > cpu.fr[15]); // fcmp/gt fr15,fr0
    if !cpu.t() {
        return; // bf 0x8c049234
    }

    cpu.r[0] = mem.read_u32(0x8C049248).unwrap(); // 0x299
    cpu.r[1] = 1;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(base), cpu.r[1] as u8);
    call_326a0(cpu, mem, base, 0x8C049250, 0x8C0491BC);
    // bra 0x8c049234
}

/// 0x8c0491c0 — the second-state flag is set, consult the descriptor bit.
fn path_b2(cpu: &mut Cpu, mem: &mut Memory, base: u32) {
    cpu.r[1] = mem.read_u32(base.wrapping_add(44)).unwrap();
    cpu.r[0] = mem.read_u32(0x8C049268).unwrap(); // 0x100
    cpu.r[1] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap();
    cpu.r[1] &= cpu.r[0]; // and r0,r1
    cpu.set_t(cpu.r[1] == 0); // tst r1,r1
    if !cpu.t() {
        // 0x8c0491ce: bit 0x100 set.
        cpu.r[0] = mem.read_u32(0x8C04926C).unwrap(); // 0x29a
        cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(base)).unwrap() as i8 as i32 as u32;
        cpu.r[0] &= 0xFF; // extu.b r0,r0
        cpu.r[0] &= 0xFF; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            return; // bf 0x8c049234
        }
        // 0x8c0491da: third flag clear, compare the 0x8c2ab460 counter to 2.
        cpu.r[0] = mem.read_u32(0x8C049270).unwrap(); // 0x8c2ab460
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.r[0] = cpu.r[0].wrapping_add(76);
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap();
        cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
        if cpu.t() {
            call_326a0(cpu, mem, base, 0x8C049250, 0x8C0491F6);
            // bra 0x8c04920a
        } else {
            call_326a0(cpu, mem, base, 0x8C049250, 0x8C04920A);
        }
        cpu.r[0] = mem.read_u32(0x8C04926C).unwrap(); // 0x29a
        cpu.r[1] = 1;
        let _ = mem.write_u8(cpu.r[0].wrapping_add(base), cpu.r[1] as u8);
        // bra 0x8c049234
    } else {
        // 0x8c049212: bit 0x100 clear, fire only from third flag == 1.
        cpu.r[0] = mem.read_u32(0x8C04926C).unwrap(); // 0x29a
        cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(base)).unwrap() as i8 as i32 as u32;
        cpu.r[0] &= 0xFF; // extu.b r0,r0
        cpu.r[0] &= 0xFF; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if !cpu.t() {
            return; // bf 0x8c049234
        }
        call_326a0(cpu, mem, base, 0x8C049250, 0x8C04922C);
        cpu.r[0] = mem.read_u32(0x8C04926C).unwrap(); // 0x29a
        cpu.r[1] = 0;
        let _ = mem.write_u8(cpu.r[0].wrapping_add(base), cpu.r[1] as u8);
    }
}

/// Shared `jsr 0x8c0326a0` prologue: `fr4 = [base + *value_pool]`,
/// `r4 = [base + 0x190]`. The literal pools hold the float-field offset, the
/// 0x190 pointer offset and the target address.
fn call_326a0(cpu: &mut Cpu, mem: &mut Memory, base: u32, value_pool: u32, return_pc: u32) {
    cpu.r[0] = mem.read_u32(value_pool).unwrap();
    cpu.r[1] = base;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8C049254).unwrap(); // 0x190
    cpu.fr[4] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[1] = mem.read_u32(0x8C049258).unwrap(); // 0x8c0326a0
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(base)).unwrap();
    cpu.pr = return_pc;
    crate::registry::p_8c0326a0::run(cpu, mem);
}
