//! 0x8c02c8d4 — clamp/accumulate a scalar field of a struct in r4.
//!
//! Straight-line code with forward-only branches; the C hides several register
//! side effects the port reproduces exactly:
//!   * r4 is left advanced (`base+0x460` on the early path, `base+0x468` on the
//!     others) — it is an in/out cursor, not just an argument,
//!   * r6 keeps `base+0x460` and is the store address in the `rts` delay slot of
//!     the divide path (`fmov fr0,@r6` writes 0.0 at [base+0x460]),
//!   * the `bra`'s delay slot clamps [base+0x460] to 1.0 only when the sum
//!     exceeded 1.0 (`bf` is not delayed, `bra` is),
//!   * T is left at the last comparison of whichever path ran.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    // PC-relative float literals, read from the image exactly like `mova`+`fmov`.
    let half = mem.read_f32(0x8c02c9f4).unwrap(); // 0.5f
    let three_tenths = mem.read_f32(0x8c02ca04).unwrap(); // 0.3f

    cpu.r[1] = 0x46c; // mov.l 0x8c02c9e0,r1
    cpu.r[0] = base;
    cpu.fr[2] = 0.0; // fldi0 fr2
    cpu.r[0] = base.wrapping_add(0x46c);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.set_t(cpu.fr[0] == cpu.fr[2]); // fcmp/eq fr2,fr0
    if cpu.t() {
        // [base+0x46c] == 0: clear flag and both floats, return with r4 advanced.
        cpu.r[0] = 0x464;
        cpu.r[5] = 0;
        cpu.r[3] = 0x468;
        cpu.r[1] = base;
        let _ = mem.write_u8(base.wrapping_add(0x464), cpu.r[5] as u8);
        cpu.r[1] = base.wrapping_add(0x468);
        cpu.r[0] = 0x460;
        let _ = mem.write_f32(cpu.r[1], cpu.fr[2]);
        cpu.r[4] = base.wrapping_add(0x460);
        let _ = mem.write_f32(cpu.r[4], cpu.fr[2]); // rts delay slot
        return;
    }

    // 0x8c02c8f8: compare [base+0x3a4] with 0.5.
    cpu.r[0] = 0x3a4;
    cpu.r[1] = base.wrapping_add(0x3a4);
    cpu.r[0] = 0x8c02c9f4;
    cpu.fr[1] = mem.read_f32(base.wrapping_add(0x3a4)).unwrap();
    cpu.fr[0] = half;
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
    if cpu.t() {
        // Path A (weight > 0.5): value += delta, clamp to 1.0 if it exceeded.
        cpu.r[0] = 0x460;
        cpu.r[3] = base;
        cpu.r[1] = 0x3e0;
        cpu.r[3] = base.wrapping_add(0x460);
        cpu.r[0] = base.wrapping_add(0x3e0);
        cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap();
        cpu.fr[0] = 1.0; // fldi1 fr0
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
        cpu.fr[2] = cpu.fr[2] + cpu.fr[1];
        let _ = mem.write_f32(cpu.r[3], cpu.fr[2]);
        cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
        cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
        if cpu.t() {
            let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // bra delay slot: clamp to 1.0
        }
    } else {
        // Path B (weight <= 0.5): value -= delta, clamp to 0.0 if negative.
        cpu.r[0] = 0x460;
        cpu.r[3] = base;
        cpu.r[1] = 0x3e0;
        cpu.r[3] = base.wrapping_add(0x460);
        cpu.r[0] = base.wrapping_add(0x3e0);
        cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
        let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
        cpu.set_t(cpu.fr[2] > cpu.fr[0]); // fcmp/gt fr0,fr2 (fr2 is still 0.0)
        if cpu.t() {
            let _ = mem.write_f32(cpu.r[3], cpu.fr[2]); // clamp to 0.0
        }
    }

    // 0x8c02c944: if value < 1.0, clear flag+float and return.
    cpu.r[0] = 0x460;
    cpu.r[6] = base;
    cpu.fr[0] = 1.0;
    cpu.r[6] = base.wrapping_add(0x460);
    cpu.fr[1] = mem.read_f32(cpu.r[6]).unwrap();
    cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
    if cpu.t() {
        cpu.r[0] = 0x464;
        cpu.r[1] = 0;
        cpu.fr[0] = 0.0;
        let _ = mem.write_u8(base.wrapping_add(0x464), 0);
        cpu.r[0] = 0x468;
        cpu.r[4] = base.wrapping_add(0x468);
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // rts delay slot
        return;
    }

    // 0x8c02c962: diff = [base+0x474] - [base+0x470]; proceed only if diff > 0.3.
    cpu.r[0] = 0x474;
    cpu.r[5] = base.wrapping_add(0x474);
    cpu.r[3] = 0x470;
    cpu.r[1] = base.wrapping_add(0x470);
    cpu.r[0] = 0x8c02ca04;
    cpu.fr[2] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.fr[0] = three_tenths;
    cpu.fr[2] = cpu.fr[2] - cpu.fr[1];
    cpu.set_t(cpu.fr[2] > cpu.fr[0]); // fcmp/gt fr0,fr2
    if !cpu.t() {
        return; // bf 0x8c02c9da: rts with nop delay slot, no further stores
    }

    // 0x8c02c97c: flag byte [base+0x410] == 1?
    cpu.r[0] = 0x410;
    let b = mem.read_u8(base.wrapping_add(0x410)).unwrap() as i8 as i32 as u32;
    cpu.r[0] = b & 0xFF; // extu.b r0,r0
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.t() {
        cpu.r[0] = 0x464;
        cpu.r[1] = 0;
        cpu.fr[0] = 0.0;
        let _ = mem.write_u8(base.wrapping_add(0x464), 0);
        cpu.r[0] = 0x468;
        cpu.r[4] = base.wrapping_add(0x468);
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // rts delay slot
        return;
    }

    // 0x8c02c998: re-test [base+0x3a4] > 0.5, then [base+0x3b8] > 0.
    cpu.r[0] = 0x3a4;
    cpu.r[3] = base.wrapping_add(0x3a4);
    cpu.r[0] = 0x8c02c9f4;
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.fr[0] = half;
    cpu.set_t(cpu.fr[1] > cpu.fr[0]);
    if !cpu.t() {
        clear_and_return(cpu, mem, base);
        return;
    }
    cpu.r[0] = 0x3b8;
    cpu.r[0] = mem.read_u32(base.wrapping_add(0x3b8)).unwrap();
    cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
    if !cpu.t() {
        clear_and_return(cpu, mem, base);
        return;
    }

    // Deepest path: flag = 1, [base+0x468] = ([base+0x3a4]-0.5)/0.5, rts delay
    // stores 0.0 at [r6] = [base+0x460].
    cpu.r[0] = 0x464;
    cpu.r[1] = 1;
    cpu.fr[0] = 0.0;
    let _ = mem.write_u8(base.wrapping_add(0x464), 1);
    cpu.r[0] = 0x8c02c9f4;
    cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[1] = 0x468;
    cpu.fr[2] = cpu.fr[2] - cpu.fr[1];
    cpu.r[4] = base.wrapping_add(0x468);
    cpu.fr[2] = cpu.fr[2] / cpu.fr[1]; // fdiv fr1,fr2
    let _ = mem.write_f32(cpu.r[4], cpu.fr[2]);
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]); // rts delay slot: fmov fr0,@r6
}

/// 0x8c02c9cc: clear flag byte and float, advance r4 — plain fall-through `rts`.
fn clear_and_return(cpu: &mut Cpu, mem: &mut Memory, base: u32) {
    cpu.r[0] = 0x464;
    cpu.r[1] = 0;
    cpu.fr[0] = 0.0;
    let _ = mem.write_u8(base.wrapping_add(0x464), 0);
    cpu.r[0] = 0x468;
    cpu.r[4] = base.wrapping_add(0x468);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // normal instruction, not a delay slot
}
