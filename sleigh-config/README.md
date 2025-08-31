# Overview

This crate contains precompiled Sleigh configuration files for use with
[libsla](https://crates.io/crates/libsla) or other crates that interface with the
[Ghidra](https://github.com/NationalSecurityAgency/ghidra) decompiler. The processor specification
files are sourced from Ghidra version **11.4**.

The configuration files are in modules based on the processor name, e.g. `processor_x86`. Which
processor modules are built is controlled via feature flags. Specify the processor name in the
feature flag to build the processor configuration files.

## Processors

The following are the list of processor configurations available in Ghidra.

* 6502
* 68000
* 8048
* 8051
* 8085
* AARCH64
* ARM
* Atmel
* BPF
* CP1600
* CR16
* DATA
* Dalvik
* HCS08
* HCS12
* JVM
* Loongarch
* M16C
* M8C
* MC6800
* MCS96
* MIPS
* PA-RISC
* PIC
* PowerPC
* RISCV
* Sparc
* SuperH
* SuperH4
* TI_MSP430
* Toy
* V850
* Xtensa
* Z80
* eBPF
* tricore
* x86

# Example

## Cargo.toml

Specify the processor(s) you wish to use via feature flags.

```toml
sleigh-config = { version = "0.1", features = ["x86"] }
```

Then use a crate that uses these configuration files to interact with Ghidra Sleigh:

```rust
// Construct new sleigh instance
let sleigh = libsla::GhidraSleigh::builder()
    .processor_spec(sleigh_config::processor_x86::PSPEC_X86_64)?
    .build(sleigh_config::processor_x86::SLA_X86_64)?;
```

# FAQ

## Why are the SLA configuration files not XML?

In Ghidra 11.1 the SLA configuration file format was changed to a binary format.
