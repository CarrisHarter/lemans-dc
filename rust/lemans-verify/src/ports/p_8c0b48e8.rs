//! 0x8c0b48e8 — one-shot initializer for the 0x54-byte record at `r4`.
//!
//! `if (record->flag != 0) return;` otherwise it builds the 3x3 matrix at
//! `record+0x24` from the four floats at `record+4` (0x8c0b2e38), copies the
//! three words at `record+0x14` to `record+0x48` (0x8c0b2f68), and — unless the
//! single-precision scalar at `record+0x20` is exactly 1.0 — scales the 3x3
//! blocks at `record+0x24`, `record+0x30` and `record+0x3c` by it (0x8c0b4ef0),
//! reloading the scalar into fr4 before each call. Finally `record->flag = 1`.
//!
//! Each `jsr @r0` enters its callee with `r4` already advanced by the delay
//! slot (+36 twice, then +36/+48/+60) and with `pr` set to the return address.
//! The epilogue reloads the pushed `pr`/`r14`, so both end at their seed values
//! and `r15` is balanced. `fcmp/eq` (no delay slot) and the `tst` before the
//! early `bf` are the only T-bit writes.

use sh4_oracle::{Cpu, Memory};

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0b4942 lds.l @r15+,pr ; 8c0b4944 rts ; 8c0b4946 mov.l @r15+,r14 (delay).
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0b48e8 push r14 ; 8c0b48ea push pr ; 8c0b48ec r14 = r4.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];

    // 8c0b48ee..8c0b48f6 — r0 = (u8)record[0] (mov.b sign-extends, extu.b
    // clears); T = (r0 == 0); bf 0x8c0b4942 when already initialized.
    cpu.r[0] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        epilogue(cpu, mem);
        return;
    }

    // 8c0b48f8..8c0b4900 — 0x8c0b2e38(record+0x24, record+4); delay: r4 += 36.
    cpu.r[0] = mem.read_u32(0x8c0b4948).unwrap_or(0);
    cpu.r[5] = cpu.r[14].wrapping_add(4);
    cpu.r[4] = cpu.r[14];
    cpu.pr = 0x8c0b4902;
    cpu.r[4] = cpu.r[4].wrapping_add(36); // delay slot of `jsr`
    crate::registry::p_8c0b2e38::run(cpu, mem);

    // 8c0b4902..8c0b490c — 0x8c0b2f68(record+0x24, record+0x14); delay: r4 += 36.
    cpu.r[0] = mem.read_u32(0x8c0b494c).unwrap_or(0);
    cpu.r[5] = cpu.r[14].wrapping_add(20);
    cpu.r[4] = cpu.r[14];
    cpu.pr = 0x8c0b490e;
    cpu.r[4] = cpu.r[4].wrapping_add(36); // delay slot of `jsr`
    crate::registry::p_8c0b2f68::run(cpu, mem);

    // 8c0b490e..8c0b4916 — fr4 = [record+0x20]; fr0 = 1.0; T = (fr0 == fr4).
    // 8c0b4918 bt 0x8c0b493e skips all three scaling calls when the scalar is 1.
    cpu.r[0] = cpu.r[14].wrapping_add(32);
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] = 1.0; // fldi1 fr0
    cpu.set_t(cpu.fr[0] == cpu.fr[4]); // fcmp/eq fr0,fr4

    if !cpu.t() {
        // 8c0b491a..8c0b4920 — 0x8c0b4ef0(record+0x24); delay: r4 += 36.
        cpu.r[0] = mem.read_u32(0x8c0b4950).unwrap_or(0);
        cpu.r[4] = cpu.r[14];
        cpu.pr = 0x8c0b4922;
        cpu.r[4] = cpu.r[4].wrapping_add(36); // delay slot of `jsr`
        crate::registry::p_8c0b4ef0::run(cpu, mem);

        // 8c0b4922..8c0b492c — fr4 = [record+0x20]; 0x8c0b4ef0(record+0x30);
        // delay: r4 += 48.
        cpu.r[0] = cpu.r[14].wrapping_add(32);
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14];
        cpu.r[0] = mem.read_u32(0x8c0b4950).unwrap_or(0);
        cpu.pr = 0x8c0b4930;
        cpu.r[4] = cpu.r[4].wrapping_add(48); // delay slot of `jsr`
        crate::registry::p_8c0b4ef0::run(cpu, mem);

        // 8c0b4930..8c0b493c — fr4 = [record+0x20]; 0x8c0b4ef0(record+0x3c);
        // delay: r4 += 60.
        cpu.r[0] = cpu.r[14].wrapping_add(32);
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14];
        cpu.r[0] = mem.read_u32(0x8c0b4950).unwrap_or(0);
        cpu.pr = 0x8c0b493e;
        cpu.r[4] = cpu.r[4].wrapping_add(60); // delay slot of `jsr`
        crate::registry::p_8c0b4ef0::run(cpu, mem);
    }

    // 8c0b493e mov #1,r0 ; 8c0b4940 mov.b r0,@r14 — record->flag = 1.
    cpu.r[0] = 1;
    let _ = mem.write_u8(cpu.r[14], 1);

    epilogue(cpu, mem);
}
