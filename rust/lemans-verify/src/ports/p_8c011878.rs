//! 0x8c011878 — strspn-style token scan over a global cursor.
//!
//! `r4` (non-null) replaces the global cursor at 0x8c23d898. A null cursor
//! returns 0. Otherwise: while the cursor byte is non-zero, scan `r5` for a
//! byte equal to the cursor's current byte and advance the cursor while
//! found. Then if the cursor stopped at NUL, clear it and return 0; else
//! call 0x8c011374 with r4 = cursor, write the result back to the cursor
//! slot, and when the result is non-zero truncate the found position with
//! NUL (r13 = 0). Key trap: `bra` at 0x8c011896 skips the `add #1,r4` at
//! 0x89a, and its delay slot `mov r13,r4` resets r4 = r13 = 0 — r13 is
//! never incremented.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = mem.read_u32(0x8c0118f8).unwrap_or(0);
    if t {
        // bt taken: skip the store
    } else {
        let _ = mem.write_u32(cpu.r[14], cpu.r[4]);
    }
    cpu.r[2] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    let t2 = cpu.r[2] == 0;
    cpu.set_t(t2);
    if t2 {
        cpu.r[0] = 0;
        epilogue(cpu, mem);
        return;
    }
    cpu.r[13] = 0; // mov #0,r13 (delay slot of bra to 0x8c0118bc)
    // second loop at 0x8c0118bc
    loop {
        cpu.r[3] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        cpu.r[2] = mem.read_u8(cpu.r[3]).unwrap_or(0) as u32;
        let tz3 = cpu.r[2] == 0;
        cpu.set_t(tz3);
        if tz3 {
            break;
        }
        cpu.r[4] = cpu.r[13]; // mov r13,r4 (delay slot of bra 0x896); the add at 0x89a is skipped
        // re-enter first loop at 0x8c01189c
        loop {
            cpu.r[0] = cpu.r[5];
            cpu.r[6] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as u32;
            let tz4 = cpu.r[6] == 0;
            cpu.set_t(tz4);
            if tz4 {
                break;
            }
            cpu.r[2] = mem.read_u32(0x8c0118f8).unwrap_or(0);
            cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
            cpu.r[3] = mem.read_u8(cpu.r[3]).unwrap_or(0) as u32;
            let eq2 = cpu.r[3] == cpu.r[6];
            cpu.set_t(eq2);
            if !eq2 {
                cpu.r[4] = cpu.r[4].wrapping_add(1);
                continue;
            }
            break;
        }
        cpu.r[0] = cpu.r[5];
        cpu.r[3] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as u32;
        let tz5 = cpu.r[3] == 0;
        cpu.set_t(tz5);
        if !tz5 {
            cpu.r[2] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.r[2] = cpu.r[2].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[14], cpu.r[2]);
        }
    }
    cpu.r[12] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[2] = mem.read_u8(cpu.r[12]).unwrap_or(0) as u32;
    let tz6 = cpu.r[2] == 0;
    cpu.set_t(tz6);
    if tz6 {
        let _ = mem.write_u32(cpu.r[14], cpu.r[13]);
        cpu.r[0] = cpu.r[13];
        epilogue(cpu, mem);
        return;
    }
    cpu.r[2] = mem.read_u32(0x8c0118fc).unwrap_or(0);
    cpu.r[4] = cpu.r[12]; // delay slot of `jsr`
    crate::registry::p_8c011374::run(cpu, mem);
    cpu.r[3] = cpu.r[0];
    let tz7 = cpu.r[3] == 0;
    cpu.set_t(tz7);
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // delay slot of `bt.s`
    if !tz7 {
        cpu.r[3] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        let _ = mem.write_u8(cpu.r[3], cpu.r[13] as u8);
    }
    cpu.r[2] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[14], cpu.r[2]);
    cpu.r[0] = cpu.r[12];
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
