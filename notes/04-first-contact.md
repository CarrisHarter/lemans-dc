# First contact with the real binary (2026-11-11)

Region: **USA** — `Test Drive Le Mans (USA) (En,Fr,Es)` (the permission names the
Japanese SKU; every offset here is US).

## Disc

`IP.BIN` (disc sector 0 of the high-density area) parses cleanly:

```
magic      SEGA SEGAKATANA
device     824D GD-ROM1/1
product    T15123N
version    V1.000
date       20000912
boot file  1ST_READ.BIN
company    SEGA LC-T-151
title      TEST DRIVE LE MANS
volume id  LE_MANS_24HR
```

Filesystem: **40 files, spanning two data tracks** (track 3 + track 5, with an
audio track between them). GD-ROM LBAs are disc-absolute: root directory sits at
LBA 45020, i.e. 20 sectors into the track-3 image. Track 5 starts at LBA 423396
(verified: `GINSU` appears exactly where `GINSU.INI`'s extent predicts).

Notable files:

| file | size | note |
| --- | --- | --- |
| `1ST_READ.BIN` | 108,376 | boot stub, **no Sega header**, raw code at 0x8C010000 |
| `LEMANS24.BIN` | 2,283,504 | **the game** — 5,213 functions |
| `DATA.ZIP` | 35,944,570 | assets |
| `LEMANS.SFD` / `INFOGRAMES.SFD` | 33.9 MB / 2.4 MB | movies |
| `MUSIC*.STREAM`, `MUSICC*.STREAM` | 100+ MB | streamed audio |
| `*.KAT` | — | sound/music banks (Katana) |
| `TRACKS.TXT`, `TEAMS.TXT`, `CHAMPIONSHIPS.TXT`, `E3TEAMS.TXT` | — | plain-text game data |
| `CLONEMAPDATABASE.CLONE_BIN/.TXT` | 100 KB / 318 KB | geometry? |
| `PADDATA.DA`, `WARN_ENG.DA` | 812 KB / 708 KB | streamed audio |

## Analysis

`rea inspect` on `LEMANS24.BIN` (raw-binary, sh4, 0x8C010000) → **5,213 procedures,
5,612 strings** in 120 s, one segment `0x8c010000-0x8c23d7f0`.

The build **keeps its debug strings**, and they leak function names directly:

```
amStreamSetVolume - trackNumber is out of range
_amStreamSetMonoVoiceChannel - trackNumber is out of range
Unclipped Track: %i soft verts, %i eles, %i nodes
Partially Clipped Track: %i soft verts, %i eles, %i nodes
loading track and building hierarchy LOD
loading track objects manager
```

That is a large head start: engine routine names (`amStream*`, track LOD building,
track object manager) can be attached to addresses by locating the string and its
referencing function, instead of being guessed.

## Target decision

`1ST_READ.BIN` is a stub; **`LEMANS24.BIN` is the analysis target** (2.28 MB).
`1ST_READ.BIN` stays useful as the loader that shows how `LEMANS24.BIN` and
`DATA.ZIP` are brought into memory.
