# Spec format

One JSON file per original function: `notes/specs/<address>.json`.
A spec is the input both sides run: the oracle executes the original bytes, the
Rust port runs `registry::<fn>`, and every observable result must match exactly.

```json
{
  "entry": "0x8c175c72",
  "regs": { "r4": "0x8c200000" },
  "fr":   { "f0": 1.5 },
  "memory": [ { "addr": "0x8c200038", "hex": "01000000" } ],
  "steps": 1000
}
```

| field | meaning |
| --- | --- |
| `entry` | function entry; defaults to the mapping base |
| `regs` | `r0`..`r15` seeds; hex strings or numbers |
| `fr` | `f0`..`f15` seeds, single precision; a hex string seeds the raw bits (the only way to give a function a NaN payload) |
| `memory` | byte patches applied before running (`hex` is raw bytes) |
| `steps` | instruction budget; exceeding it is a failure, not a pass |
| `sr`, `mach`, `macl`, `gbr`, `vbr`, `ssr`, `spc`, `sgr`, `dbr`, `fpul`, `fpscr` | optional system-register seeds, hex strings |

## What is compared

- all 16 integer registers
- all 16 FPU registers, bit-exact (no tolerance — a 1-ULP drift compounds)
- the full `sr`, plus `mach`, `macl`, `gbr`, `vbr`, `ssr`, `spc`, `sgr`, `dbr`,
  `fpul` and `fpscr`
- every store outside the synthetic stack window

The stack window (`0x8C300000`..`0x8C310000`) is excluded on purpose: a port may
spill differently and still be correct. Anything the port writes elsewhere must
match the original byte for byte.

## Not yet modelled

- `div1` and the vector FPU instructions (`fipr`, `ftrv`, `fschg`, `frchg`,
  `fsca`, `fsrra`, `fcnvsd`) — they fail loudly as `Unimplemented` rather than
  running with unverified semantics
- calls into other subsystems (a spec must stay inside one function's own work)
- null/error branches that call the engine's reporter (documented per port)
- DMA, store queues, interrupts, and any hardware register access

## Evidence

`scripts/verify-function.sh <address>` runs both sides and writes
`notes/verifications/<address>.json`: result, oracle step count, binary sha256,
and the commit that introduced the port. A name in `notes/address-map.md` marked
verified must have one of those files.

## Heap fixtures (free/malloc paths)

`free` at `0x8c159fc0` reads the block header at `p-8`, negates the size word and
walks to `p - size + 8` to inspect the next chunk, then links the block into a
bin through the top pointer at `0x8c2b1ef8`. A spec therefore needs **numeric
values that survive that walk**, not plausible-looking big ones:

| address | value | why |
| --- | --- | --- |
| `p-8` | `0x8c220000` | prev_size; only read if the prev-in-use bit is clear |
| `p-4` | `0x40` | block size; 64 bytes keeps `p - size` inside the image |
| `p-0x38` (= next-4) | `8` | next chunk's size; large values make the bin write land outside mapped memory |
| `0x8c2b1ef8` | `0x8c210000` | top-chunk pointer |
| `0x8c210000` | `0x1000` | top-chunk size |

Values are raw little-endian bytes: `0x40` is `"40000000"`, `8` is
`"08000000"`, `0x8c210000` is `"0000218c"`. Writing `"00000040"` stores
0x40000000, which makes `free` walk to an unmapped chunk and the oracle dies with
`data access ... outside mapped memory` before the port is compared.
