# RP2350 Secure OTA System

A secure and resilient **Over-The-Air (OTA) firmware update system** for the RP2350 (Raspberry Pi Pico 2W), designed for real-world embedded deployments where reliability and security are critical.

> This is **not just a code example**.
> This repository focuses on **system design, architecture, and decision-making** before implementation.

---

## 🚀 Project Goal

Design and implement a **fault-tolerant and secure OTA update system** that:

* Never bricks the device
* Survives power loss during updates
* Guarantees firmware authenticity and integrity
* Supports rollback using dual firmware slots (A/B)
* Leverages RP2350 hardware security features

---

## 🧠 Engineering Approach

This project follows a **design-first methodology**:

1. **Conception** → Requirements, constraints, threat model
2. **Architecture** → Memory layout, boot flow, system states
3. **Design Decisions** → Documented using ADRs
4. **Implementation** → Incremental and validated

All major decisions are documented under:

```
docs/adr/
```

---

## 🔐 Key Features (Planned)

* Secure boot integration (leveraging RP2350 Boot ROM)
* A/B firmware slots with rollback capability
* Cryptographic verification (SHA-256 + digital signatures)
* Metadata-based boot decision system
* Power-loss resilient update mechanism
* OTA over WiFi (planned)
* Hardware acceleration usage (SHA engine)

---

## 🧱 System Overview (High-Level)

```
Boot ROM (Stage 1 - Hardware)
        ↓
Stage 2 Bootloader
        ↓
Metadata Evaluation
        ↓
Slot Selection (A/B)
        ↓
Firmware Execution
```

OTA Flow:

```
Download → Write (inactive slot) → Verify → Commit → Reboot
```

---

## ⚙️ Tech Stack

* Rust (`no_std`)
* Async runtime: Embassy
* Logging: defmt + RTT
* Target: RP2350 (Raspberry Pi Pico 2W)

---

## 📂 Repository Structure

```
.
├── src/                # Firmware source code
├── docs/
│   └── adr/            # Architecture Decision Records
├── memory.x            # Linker script
├── Cargo.toml
└── README.md
```

---

## 🧪 Current Status

🚧 Early stage — architecture and design phase

Initial goals:

* Establish boot strategy
* Define memory layout
* Design metadata system
* Validate hardware capabilities

---

## 🎯 Why This Project?

Modern embedded systems require:

* Remote updates
* Strong security guarantees
* High reliability in harsh environments

This project explores how to build such a system **from first principles**, leveraging both hardware and software design.

---

## 🤖 Development Strategy

This project is intentionally developed using **multiple AI-assisted perspectives**, with all decisions reviewed and validated by the engineer.

The goal is not automation, but **better engineering judgment**.

---

## 📌 Future Work

* Define OTA communication protocol
* Implement cryptographic validation pipeline
* Integrate WiFi stack
* Add rollback protection using OTP
* Build signing toolchain

---

## 📜 License

MIT / Apache-2.0 (TBD)
