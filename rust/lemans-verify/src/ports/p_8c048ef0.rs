//! 0x8c048ef0 — state-gated count advance with a 12-byte state switch.
//!
//! `r4` points at a control block: `+0x190` is a struct, `+0x23c` a mode word,
//! `+0x244` a counter, `+0x248` a companion word and `+0x2a0` a flag word.
//! Two floats on the struct (`+0x10` when mode == 0x200, `+0x0c` otherwise)
//! decide whether the routine runs at all; when it does, it calls
//! 0x8c151b1c to advance the object at `struct+0x24` by 108 and inspects the
//! float it lands on. A value below 1.5 bumps the counter and, once the
//! counter exceeds one, switches the mode word by copying one of two 12-byte
//! tables (0x8c1f6680 when struct+0x40 is negative, 0x8c1f6690 otherwise).
//!
//! The callee is invoked with its ABI return address in `pr`; the routine
//! restores `pr` (along with r12/r13/r14) from its own frame, so only the
//! caller-saved results and the FPU scratch registers survive.

use sh4_oracle::{Cpu, Memory};

/// Force the counter to zero and return 0 (0x8c048fce / 0x8c048fd4).
fn reset(cpu: &mut Cpu, mem: &mut Memory, field: u32, pr: u32) {
    cpu.r[0] = 0x244;
    cpu.r[1] = 0;
    let _ = mem.write_u32(field.wrapping_add(0x244), 0);
    cpu.r[0] = 0;
    cpu.pr = pr;
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let pr = cpu.pr;
    let field = cpu.r[4]; // r13
    let obj = mem.read_u32(field.wrapping_add(0x190)).unwrap_or(0); // r14

    cpu.r[1] = 0x200;
    cpu.r[3] = mem.read_u32(field.wrapping_add(0x23c)).unwrap_or(0);

    // 0x8c048f04..0x8c048f24: pick the gate float from the mode word.
    let call;
    if cpu.r[3] == 0x200 {
        cpu.r[0] = obj.wrapping_add(0x10);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[0] = 0.0;
        cpu.set_t(cpu.fr[1] > 0.0);
        call = cpu.t();
    } else {
        call = false;
    }
    if !call {
        cpu.r[0] = 0x200;
        cpu.set_t(cpu.r[0] == cpu.r[3]);
        if cpu.t() {
            reset(cpu, mem, field, pr);
            return;
        }
        cpu.r[0] = obj.wrapping_add(0x0c);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[0] = 0.0;
        cpu.set_t(cpu.fr[1] > 0.0);
        if !cpu.t() {
            reset(cpu, mem, field, pr);
            return;
        }
    }

    // 0x8c048f26: jsr @0x8c151b1c, argument loaded in the delay slot.
    cpu.r[0] = 0x8c151b1c;
    cpu.r[4] = mem.read_u32(obj.wrapping_add(0x24)).unwrap_or(0);
    cpu.pr = 0x8c048f2c;
    crate::registry::p_8c151b1c::run(cpu, mem);

    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr1
    cpu.r[0] = 0x8c048ff0; // mova
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 1.5f
    cpu.set_t(cpu.fr[0] > cpu.fr[1]);
    if !cpu.t() {
        reset(cpu, mem, field, pr);
        return;
    }

    // 0x8c048f36: bump the counter unless the flag word is clear.
    cpu.r[0] = 0x2a0;
    cpu.r[0] = mem.read_u32(field.wrapping_add(0x2a0)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = 0x244;
        cpu.r[1] = mem
            .read_u32(field.wrapping_add(0x244))
            .unwrap_or(0)
            .wrapping_add(1);
        let _ = mem.write_u32(field.wrapping_add(0x244), cpu.r[1]);
    }
    cpu.r[3] = 0x244;
    cpu.r[1] = 1;
    cpu.r[0] = mem.read_u32(field.wrapping_add(0x244)).unwrap_or(0);
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32));
    if !cpu.t() {
        cpu.r[0] = 0;
        cpu.pr = pr;
        return;
    }

    // 0x8c048f52: the switch runs only after a second bump.
    cpu.fr[1] = mem.read_f32(obj.wrapping_add(0x40)).unwrap_or(0.0);
    cpu.fr[0] = 0.0;
    cpu.set_t(0.0 > cpu.fr[1]);
    if cpu.t() {
        // Struct value is negative: switch to mode 4 via table 0x8c1f6680.
        cpu.r[0] = 0x2a0;
        cpu.r[0] = mem.read_u32(field.wrapping_add(0x2a0)).unwrap_or(0) & 8;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.t() {
            cpu.r[0] = 0;
            cpu.pr = pr;
            return;
        }
        cpu.r[0] = 0x23c;
        cpu.r[4] = 0x200;
        cpu.r[5] = mem.read_u32(field.wrapping_add(0x23c)).unwrap_or(0);
        cpu.set_t(cpu.r[4] == cpu.r[5]);
        if !cpu.t() {
            cpu.r[0] = 0;
            cpu.pr = pr;
            return;
        }
        cpu.r[7] = 0x8c1f6680;
        cpu.r[4] = mem.read_u32(cpu.r[7]).unwrap_or(0);
        cpu.r[6] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0);
        let _ = mem.write_u32(field.wrapping_add(0x230), cpu.r[4]);
        let _ = mem.write_u32(field.wrapping_add(0x234), cpu.r[6]);
        cpu.r[4] = 0;
        cpu.r[6] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0);
        let _ = mem.write_u32(field.wrapping_add(0x238), cpu.r[6]);
        cpu.r[5] = 4;
        let _ = mem.write_u32(field.wrapping_add(0x23c), cpu.r[5]);
        cpu.r[0] = 0x244;
        let _ = mem.write_u32(field.wrapping_add(0x244), cpu.r[4]);
        cpu.r[0] = 0x248;
        let _ = mem.write_u32(field.wrapping_add(0x248), cpu.r[4]);
        cpu.r[0] = cpu.r[1]; // delay slot of `bra`
        cpu.pr = pr;
        return;
    }

    // Non-negative: switch to mode 0x200 via table 0x8c1f6690.
    cpu.r[0] = 0x2a0;
    cpu.r[0] = mem.read_u32(field.wrapping_add(0x2a0)).unwrap_or(0) & 2;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = 0;
        cpu.pr = pr;
        return;
    }
    cpu.r[0] = 0x23c;
    cpu.r[4] = mem.read_u32(field.wrapping_add(0x23c)).unwrap_or(0);
    cpu.set_t(cpu.r[4] == 0x200);
    if cpu.t() {
        cpu.r[0] = 0;
        cpu.pr = pr;
        return;
    }
    cpu.r[7] = 0x8c1f6690;
    cpu.r[6] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] = 0;
    let _ = mem.write_u32(field.wrapping_add(0x230), cpu.r[6]);
    let _ = mem.write_u32(field.wrapping_add(0x234), cpu.r[5]);
    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(field.wrapping_add(0x238), cpu.r[5]);
    cpu.r[0] = 0x23c;
    let _ = mem.write_u32(field.wrapping_add(0x23c), 0x200);
    cpu.r[0] = 0x244;
    let _ = mem.write_u32(field.wrapping_add(0x244), cpu.r[4]);
    cpu.r[0] = 0x248;
    let _ = mem.write_u32(field.wrapping_add(0x248), cpu.r[4]);
    cpu.r[0] = cpu.r[1]; // delay slot of `bra`
    cpu.pr = pr;
}
