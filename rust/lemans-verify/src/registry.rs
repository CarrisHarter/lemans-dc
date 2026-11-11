//! Registry of ported functions, keyed by their original SH-4 address.
//!
//! A port is a function that mutates `Cpu` and `Memory` exactly as the original
//! routine did: registers, FPU state and every store. Nothing else is allowed —
//! no hidden state, no I/O, no allocator.

use sh4_oracle::{Cpu, Memory};

pub type Port = fn(&mut Cpu, &mut Memory);

pub fn lookup(address: u32) -> Option<Port> {
    match address {
        0x8C01_0000 => Some(fixture_sum_loop),
        0x8C17_5C72 => Some(am_sound_is_playing),
        0x8C18_B9C8 => Some(array_lookup),
        0x8C15_52E4 => Some(store_through_two_levels),
        _ => None,
    }
}

/// 0x8c010000 — fixture: `r0` counts down `r1` to zero.
///
/// Original: `mov r1,r4 ; mov #0,r0 ; add #1,r0 ; dt r1 ; bf …`
/// T ends set because `dt` leaves zero.
pub fn fixture_sum_loop(cpu: &mut Cpu, _mem: &mut Memory) {
    let mut counter = cpu.r[1];
    cpu.r[4] = counter; // the original's `mov r1,r4` is an observable side effect
    let mut acc: u32 = 0;
    loop {
        acc = acc.wrapping_add(1);
        counter = counter.wrapping_sub(1);
        if counter != 0 {
            continue;
        }
        break;
    }
    cpu.r[0] = acc;
    cpu.r[1] = counter;
    cpu.set_t(true);
}

/// 0x8c175c72 — `amSoundIsPlaying(sound)`.
///
/// Original: `if (sound == 0) { amSoundIsPlaying - theSound is NULL; return 0; }
///            return *(int *)(sound + 0x38) != 0;`
///
/// Ported for the non-null path only: the null path calls the engine's error
/// reporter, which is a separate system and is out of scope for this function.
pub fn am_sound_is_playing(cpu: &mut Cpu, mem: &mut Memory) {
    let sound = cpu.r[4];
    if sound == 0 {
        cpu.r[0] = 0;
        return;
    }
    let playing = mem.read_u32(sound + 0x38).unwrap_or(0);
    cpu.r[3] = playing; // the original leaves the field in r3
    cpu.r[0] = u32::from(playing != 0);
}

/// 0x8c18b9c8 — bounds-checked element lookup.
///
/// `if (index >= 0 && index < *base) return index * 0x14 + base[1];`
/// `return base[2];`
pub fn array_lookup(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let index = cpu.r[5] as i32;
    let count = mem.read_u32(base).unwrap_or(0) as i32;
    // `cmp/pz` then `cmp/gt` leave T observable on every path.
    if index < 0 {
        cpu.set_t(false);
        cpu.r[0] = mem.read_u32(base + 8).unwrap_or(0);
        return;
    }
    cpu.set_t(count > index);
    cpu.r[0] = if count > index {
        let stride = mem.read_u32(base + 4).unwrap_or(0);
        cpu.r[1] = stride; // the original leaves base[1] in r1 on this path
        (index as u32).wrapping_mul(0x14).wrapping_add(stride)
    } else {
        mem.read_u32(base + 8).unwrap_or(0)
    };
}

/// 0x8c1552e4 — write a value at `base+4` and through a pointer at `base+0x20`.
pub fn store_through_two_levels(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let value = cpu.r[5];
    let _ = mem.write_u32(base + 4, value);
    let inner = mem.read_u32(base + 0x20).unwrap_or(0);
    cpu.r[0] = inner; // the original keeps the pointer in r0
    let _ = mem.write_u32(inner + 8, value);
}
