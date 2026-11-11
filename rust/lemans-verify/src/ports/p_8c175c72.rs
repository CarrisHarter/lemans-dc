//! 0x8c175c72 — `amSoundIsPlaying(sound)`.
//!
//! Original: `if (sound == 0) { amSoundIsPlaying - theSound is NULL; return 0; }
//!            return *(int *)(sound + 0x38) != 0;`
//!
//! Ported for the non-null path only: the null path calls the engine's error
//! reporter, which is a separate system and is out of scope for this function.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let sound = cpu.r[4];
    if sound == 0 {
        cpu.r[0] = 0;
        return;
    }
    let playing = mem.read_u32(sound + 0x38).unwrap_or(0);
    cpu.r[3] = playing; // the original leaves the field in r3
    cpu.r[0] = u32::from(playing != 0);
}
