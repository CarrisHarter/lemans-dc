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
