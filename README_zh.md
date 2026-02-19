# rwbt

中文 | [English](README.md)

面向 [RivieraWaves](https://www.ceva-ip.com/product/ceva-waves-bluetooth/) 蓝牙 IP 核的 `no_std` Rust 库，
适用于 SiFli、博通、瑞昱、博芯科等所有使用该 IP 的芯片。

## 目录结构

```
rwbt/
├── src/
│   ├── common.rs          # Reg<T, A> 寄存器抽象（R / W / RW 权限标记）
│   ├── mac/               # BT MAC 寄存器块 — 从 YODA CSV 自动生成
│   │   ├── mod.rs
│   │   └── prebuilt.rs    # 17k 行预生成代码，零构建依赖
│   └── rfc/
│       ├── cmd.rs          # RFC 命令 ISA（跨厂商通用）
│       └── sifli/          # SiFli 射频前端
│           ├── regs.rs     #   寄存器偏移和位常量
│           └── cal_table.rs#   校准表打包
├── build.rs               # YODA CSV → Rust 代码生成
└── data/
    ├── RW_DM_CORE_REG.csv # 设备管理寄存器
    ├── RW_BT_CORE_REG.csv # 经典蓝牙寄存器
    └── RW_BLE_CORE_REG.csv# BLE 寄存器
```

### 跨厂商 vs 厂商专用

| 模块 | 适用范围 | 说明 |
|---|---|---|
| `mac::{dm, bt, ble}` | 跨厂商 | 所有 RivieraWaves 授权厂商共享的 BT MAC 寄存器块 |
| `rfc::cmd` | 跨厂商 | RFC 命令序列器指令集（`rd`/`wr`/`or`/`and`/`wait`） |
| `rfc::sifli` | 仅 SiFli | SiFli 射频前端寄存器布局和校准表格式 |

## 用法

```rust
use rwbt::mac::ble::Ble;
use rwbt::mac::dm::Dm;
use rwbt::rfc::cmd::{self, CmdBuilder};

// 访问 BLE MAC 寄存器
let ble = unsafe { Ble::from_ptr(0x4009_0800 as _) };
let version = ble.version().read();
let upg = version.upg();

// 修改寄存器
ble.rwblecntl().modify(|w| {
    w.set_rwble_en(true);
    w.set_crc_dsb(false);
});

// 构建 RFC 命令序列
let mut seq = CmdBuilder::new();
seq.push(cmd::rd(0x00));     // 读 VCO_REG1
seq.push(cmd::or(12));       // 置位 bit 12
seq.push(cmd::wr(0x00));     // 写回
seq.push(cmd::wait(10));     // 等待 10 µs
seq.push(cmd::END);
```

## Features

| Feature | 默认 | 说明 |
|---|---|---|
| `prebuild` | 是 | 使用预生成的 MAC 寄存器代码（无 build.rs 开销） |
| `sifli` | 否 | 启用 SiFli 射频前端寄存器常量和校准表打包 |

### 重新生成 MAC 寄存器代码

```bash
# 关闭 prebuild 以触发 CSV → Rust 代码生成
cargo build --no-default-features

# 生成文件位于 $OUT_DIR/mac_generated.rs
# 复制到 src/mac/prebuilt.rs 以更新提交的预生成版本
```

## 数据来源

`data/` 目录下的 CSV 文件采用 RivieraWaves YODA 寄存器导出格式，
来自 [bekmen/github_mcp_experiment](https://github.com/bekmen/github_mcp_experiment/tree/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda)：

| 文件 | 链接 |
|---|---|
| `RW_DM_CORE_REG.csv` | [DM 核心](https://github.com/bekmen/github_mcp_experiment/blob/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda/RW_DM_CORE_REG.csv) |
| `RW_BT_CORE_REG.csv` | [BT 核心](https://github.com/bekmen/github_mcp_experiment/blob/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda/RW_BT_CORE_REG.csv) |
| `RW_BLE_CORE_REG.csv` | [BLE 核心](https://github.com/bekmen/github_mcp_experiment/blob/main/trunk/YODA/subProjects/hudson_r1_trunk/yoda/RW_BLE_CORE_REG.csv) |

定义了标准 BT MAC 寄存器集（DM + BT + BLE 三块），已与 7 个独立厂商的 PAC 交叉验证，
确认地址和字段完全匹配。

## 许可证

Apache-2.0
