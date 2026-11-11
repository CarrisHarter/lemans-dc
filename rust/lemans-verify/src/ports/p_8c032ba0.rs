//! 0x8c032ba0 — apply a mode-scaled transform to a struct (r4) using a mode
//! table (r5) and a flag word (r6).
//!
//! Two independent flag tests in r6 (bits 2 and 3) gate two blocks. The C hides:
//!   * `bt.s` at entry has a delay slot (`mov.l @r1,r1`) that executes on BOTH
//!     paths, so r1 is always dereferenced through the pointer literal at
//!     0x8c032cb0 (read from the image, specs may repoint it),
//!   * each mode dispatch uses `bra` with an FPU delay slot, so the delay-slot
//!     fadd/fsub runs unconditionally once the mode block is entered,
//!   * in the bit-3 block every `bra` carries `add r0,r1` in its delay slot, so
//!     r1 (the base pointer used afterwards) is advanced by the mode offset on
//!     every path, including the fall-through default,
//!   * r4 ends advanced by 0x178 and r5 is left at r1+4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5arg = cpu.r[5];
    let r6 = cpu.r[6];

    cpu.r[1] = mem.read_u32(0x8c032cb0).unwrap(); // global pointer G
    cpu.r[0] = r6 & 4; // and #4,r0
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    // bt.s 0x8c032c44 — delay slot always runs:
    cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap(); // mov.l @r1,r1
    if cpu.r[0] == 0 {
        // bit 2 clear: skip the first block
        second_block(cpu, mem, r4, r5arg);
        return;
    }

    // First block: mode = [r5+4]
    cpu.r[0] = mem.read_u32(r5arg.wrapping_add(4)).unwrap();
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        mode_block(cpu, mem, 0); // 0x8c032bbe
    } else {
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if cpu.t() {
            mode_block(cpu, mem, 1); // 0x8c032bd2
        } else {
            cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
            if cpu.t() {
                mode_block(cpu, mem, 2); // 0x8c032be4
            } else {
                // bra 0x8c032bfe, delay nop: default block
                mode_block(cpu, mem, 3); // 0x8c032bfe
            }
        }
    }

    // 0x8c032c0e: scale three floats of r4 by fr2 / fr3.
    cpu.r[3] = mem.read_u32(0x8c032cb8).unwrap(); // 0x388
    cpu.r[0] = r4;
    cpu.r[7] = r4;
    cpu.r[0] = r4.wrapping_add(cpu.r[3]);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[3] = mem.read_u32(0x8c032cbc).unwrap(); // 0x38c
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2]; // fmul fr2,fr0
    cpu.r[0] = mem.read_u32(0x8c032cc0).unwrap(); // 0x390
    cpu.r[7] = r4.wrapping_add(cpu.r[0]);
    cpu.r[0] = r4;
    let _ = mem.write_f32(cpu.r[7], cpu.fr[0]);
    cpu.r[0] = r4.wrapping_add(cpu.r[3]);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[7] = r4;
    cpu.r[3] = mem.read_u32(0x8c032cc4).unwrap(); // 0x37c
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2];
    cpu.r[0] = mem.read_u32(0x8c032cc8).unwrap(); // 0x394
    cpu.r[7] = r4.wrapping_add(cpu.r[0]);
    cpu.r[0] = r4;
    let _ = mem.write_f32(cpu.r[7], cpu.fr[0]);
    cpu.r[0] = r4.wrapping_add(cpu.r[3]);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[3] = mem.read_u32(0x8c032ccc).unwrap(); // 0x380
    cpu.r[0] = r4;
    cpu.fr[0] = cpu.fr[0] * cpu.fr[3]; // fmul fr3,fr0 — fr3 is mode-dependent
    cpu.r[0] = r4.wrapping_add(cpu.r[3]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);

    second_block(cpu, mem, r4, r5arg);
}

/// One mode block (0x8c032bbe/0x8c032bd2/0x8c032be4/0x8c032bfe): compute fr2/fr3
/// from [r1+4] and [r1+8], with the `bra` delay slot applied unconditionally.
fn mode_block(cpu: &mut Cpu, mem: &mut Memory, mode: u32) {
    let r1 = cpu.r[1];
    match mode {
        0 => {
            // 0x8c032bbe: fr2 = 1 - [r1+8]*0.5 ; fr3 = 1.0
            cpu.r[3] = r1.wrapping_add(8);
            cpu.r[0] = 0x8c032cb4;
            cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
            cpu.fr[3] = 1.0; // fldi1 fr3
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
            cpu.fr[2] = cpu.fr[3]; // fmov fr3,fr2
            cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
            cpu.fr[2] = cpu.fr[2] - cpu.fr[1]; // bra delay slot: fsub fr1,fr2
        }
        1 => {
            // 0x8c032bd2: fr3 = [r1+8]*0.5 + 1 ; fr2 = 1.0
            cpu.r[3] = r1.wrapping_add(8);
            cpu.r[0] = 0x8c032cb4;
            cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap();
            cpu.fr[2] = 1.0; // fldi1 fr2
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
            cpu.fr[3] = cpu.fr[3] * cpu.fr[0];
            cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // bra delay slot: fadd fr2,fr3
        }
        2 => {
            // 0x8c032be4: fr2 = [r1+4]*0.5 + 1 ; fr3 = [r1+8] + 1
            cpu.r[7] = r1.wrapping_add(4);
            cpu.r[3] = r1.wrapping_add(8);
            cpu.r[0] = 0x8c032cb4;
            cpu.fr[2] = mem.read_f32(cpu.r[7]).unwrap();
            cpu.fr[0] = 1.0; // fldi1 fr0
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
            cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap();
            cpu.fr[2] = cpu.fr[2] * cpu.fr[1];
            cpu.fr[2] = cpu.fr[2] + cpu.fr[0];
            cpu.fr[3] = cpu.fr[3] + cpu.fr[0]; // bra delay slot: fadd fr0,fr3
        }
        _ => {
            // 0x8c032bfe (default): fr3 = [r1+8]*0.5 + 1 ; fr2 = 1.0
            cpu.r[3] = r1.wrapping_add(8);
            cpu.r[0] = 0x8c032cb4;
            cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap();
            cpu.fr[2] = 1.0;
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
            cpu.fr[3] = cpu.fr[3] * cpu.fr[0];
            cpu.fr[3] = cpu.fr[3] + cpu.fr[2];
        }
    }
}

/// Second block (bit 3 of r6): offset r1 by the mode table, then copy two
/// floats from r1 into r4's fields; r4 ends at r4+0x178.
fn second_block(cpu: &mut Cpu, mem: &mut Memory, r4: u32, _r5arg: u32) {
    cpu.r[0] = cpu.r[6] & 8; // and #8,r0
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        // bt 0x8c032caa: rts with nop delay slot
        return;
    }
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap();
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0);
    let off = if cpu.t() {
        mem.read_u32(0x8c032cd0).unwrap() // 0x88
    } else {
        cpu.set_t(cpu.r[0] == 1);
        if cpu.t() {
            mem.read_u32(0x8c032cd4).unwrap() // 0x98
        } else {
            cpu.set_t(cpu.r[0] == 2);
            if cpu.t() {
                mem.read_u32(0x8c032cd8).unwrap() // 0xa8
            } else {
                cpu.set_t(cpu.r[0] == 3);
                if cpu.t() {
                    mem.read_u32(0x8c032cdc).unwrap() // 0xb8
                } else {
                    mem.read_u32(0x8c032cd0).unwrap() // 0x88 (default)
                }
            }
        }
    };
    cpu.r[0] = off;
    cpu.r[1] = cpu.r[1].wrapping_add(off); // bra delay slot: add r0,r1

    // 0x8c032c7e
    cpu.r[5] = cpu.r[1].wrapping_add(4);
    cpu.r[3] = mem.read_u32(0x8c032ce0).unwrap(); // 0xe4
    cpu.r[0] = r4;
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[0] = r4.wrapping_add(cpu.r[3]);
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[3] = mem.read_u32(0x8c032ce4).unwrap(); // 0xe8
    cpu.r[0] = r4;
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[0] = r4.wrapping_add(cpu.r[3]);
    cpu.r[3] = r4;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[0] = mem.read_u32(0x8c032ce8).unwrap(); // 0x174
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[3] = r4.wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c032cec).unwrap(); // 0x178
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[4] = r4.wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
}
