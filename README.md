# rwbt

[中文](README_zh.md) | English

`no_std` Rust crate for the [RivieraWaves](https://www.ceva-ip.com/product/ceva-waves-bluetooth/) Bluetooth IP core,
found in chips from SiFli, Broadcom, Realtek, Beken, and others.

## Structure

```
rwbt/
├── src/
│   ├── common.rs              # Reg<T, A> register abstraction (R / W / RW)
│   ├── mac/                   # BT MAC register blocks — auto-generated from YODA CSV
│   │   ├── mod.rs
│   │   └── prebuilt.rs        # 17k lines, committed for zero-build-dep usage
│   ├── rfc/
│   │   ├── cmd.rs             # RFC command ISA (cross-vendor)
│   │   └── sifli/             # SiFli RF front-end
│   │       ├── regs.rs        #   register offsets & bit constants (auto-generated)
│   │       └── cal_table.rs   #   calibration table packing
│   └── bin/
│       └── gen_sifli_regs.rs  # Tool: PAC YAML → regs.rs codegen
├── build.rs                   # YODA CSV → Rust codegen
└── data/
    ├── RW_DM_CORE_REG.csv     # Device Management registers
    ├── RW_BT_CORE_REG.csv     # Classic BT registers
    └── RW_BLE_CORE_REG.csv    # BLE registers
```

### Cross-vendor vs vendor-specific

| Module | Scope | Description |
|---|---|---|
| `mac::{dm, bt, ble}` | Cross-vendor | BT MAC register blocks shared by all RivieraWaves licensees |
| `rfc::cmd` | Cross-vendor | RFC command sequencer ISA (`rd`/`wr`/`or`/`and`/`wait`) |
| `rfc::sifli` | SiFli only | RF front-end register layout and calibration table formats |

## Usage

```rust
use rwbt::mac::ble::Ble;
use rwbt::mac::dm::Dm;
use rwbt::rfc::cmd::{self, CmdBuilder};

// Access BLE MAC registers
let ble = unsafe { Ble::from_ptr(0x4009_0800 as _) };
let version = ble.version().read();
let upg = version.upg();

// Modify a register
ble.rwblecntl().modify(|w| {
    w.set_rwble_en(true);
    w.set_crc_dsb(false);
});

// Build an RFC command sequence
let mut seq = CmdBuilder::new();
seq.push(cmd::rd(0x00));     // read VCO_REG1
seq.push(cmd::or(12));       // set bit 12
seq.push(cmd::wr(0x00));     // write back
seq.push(cmd::wait(10));     // wait 10 µs
seq.push(cmd::END);
```

## Features

| Feature | Default | Description |
|---|---|---|
| `prebuild` | no | Use pre-generated MAC register code (skip build.rs codegen) |
| `sifli` | no | Enable SiFli RF front-end register constants and calibration tables |

### Regenerating MAC registers

```bash
# By default, build.rs generates MAC registers from CSV
cargo build

# The generated file is at $OUT_DIR/mac_generated.rs
# Copy it to src/mac/prebuilt.rs to update the committed version
```

### Regenerating SiFli RFC registers

`src/rfc/sifli/regs.rs` is auto-generated from the sifli-pac `bt_rfc.yaml`
(chiptool format). To regenerate after PAC updates:

```bash
cargo run --features _gen --bin gen-sifli-regs -- <path-to-bt_rfc.yaml> > src/rfc/sifli/regs.rs
rustfmt src/rfc/sifli/regs.rs
```

The `_gen` feature gates the binary tool so it is never built by library consumers.

## Data source

The CSV files in `data/` are in RivieraWaves YODA register export format,
sourced from [bekmen/github_mcp_experiment](https://github.com/bekmen/github_mcp_experiment/tree/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda):

| File | Link |
|---|---|
| `RW_DM_CORE_REG.csv` | [DM core](https://github.com/bekmen/github_mcp_experiment/blob/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda/RW_DM_CORE_REG.csv) |
| `RW_BT_CORE_REG.csv` | [BT core](https://github.com/bekmen/github_mcp_experiment/blob/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda/RW_BT_CORE_REG.csv) |
| `RW_BLE_CORE_REG.csv` | [BLE core](https://github.com/bekmen/github_mcp_experiment/blob/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda/RW_BLE_CORE_REG.csv) |

They define the standard BT MAC register set (DM + BT + BLE blocks) and have been
cross-verified against 7 independent vendor PACs to confirm address and field accuracy.

## License

Apache-2.0
