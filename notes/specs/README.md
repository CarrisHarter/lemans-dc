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
| `fr` | `f0`..`f15` seeds, single precision |
| `memory` | byte patches applied before running (`hex` is raw bytes) |
| `steps` | instruction budget; exceeding it is a failure, not a pass |

## What is compared

- all 16 integer registers
- all 16 FPU registers, bit-exact (no tolerance — a 1-ULP drift compounds)
- the T bit in `sr`
- every store outside the synthetic stack window

The stack window (`0x8C300000`..`0x8C310000`) is excluded on purpose: a port may
spill differently and still be correct. Anything the port writes elsewhere must
match the original byte for byte.

## Not yet modelled

- calls into other subsystems (a spec must stay inside one function's own work)
- null/error branches that call the engine's reporter (documented per port)
- DMA, store queues, interrupts, and any hardware register access

## Evidence

`scripts/verify-function.sh <address>` runs both sides and writes
`notes/verifications/<address>.json`: result, oracle step count, binary sha256,
and the commit that introduced the port. A name in `notes/address-map.md` marked
verified must have one of those files.
