# RP2350 Secure OTA System

A firmware update system for the RP2350 (Raspberry Pi Pico 2W), developed in Rust.

This project investigates the architecture and implementation of a secure OTA update mechanism, with emphasis on firmware image validation, flash memory management, boot control, and recovery from interrupted updates.

The initial development focuses on the boot process and storage architecture before integrating the complete wireless update pipeline. 

To ensure fault tolerance and secure image selection, the system enforces a strict separation between the update delivery mechanisms and the critical boot validation path. The following structural topology illustrates these trust boundaries and component responsibilities:

![Bootloader Structural Architecture](./docs/Bootloader_Structural_Architecture_v2.drawio.svg)

## Project Objectives

The system is being designed around the following requirements:

- **Image integrity:** detect corrupted or incomplete firmware images before execution.
- **Image authenticity:** verify that firmware originates from a trusted signing authority.
- **Update recovery:** preserve a bootable firmware image if an update is interrupted.
- **A/B image management:** write candidate firmware to an inactive storage region without overwriting the currently confirmed image.
- **Controlled boot selection:** distinguish between confirmed and candidate images, including the handling of unsuccessful trial boots.
- **Hardware security:** evaluate the security mechanisms available on the RP2350 and their applicability to the boot architecture.

These objectives will guide the architectural decisions and implementation. Their guarantees will be established through hardware-level validation and fault-injection testing.

## Design Philosophy: Hardware-Agnostic Architecture

This system is built using a strict **Hexagonal Architecture (Ports and Adapters)**. While implemented in Rust, the structural design is language-agnostic and maps directly to industry-standard C/C++ embedded paradigms.

- **MCU-Agnostic Core:** The update logic, boot state machine, and cryptography (`crates/update-core`, `crates/boot-core`) are entirely isolated from the hardware. They compile independently of the target MCU.
- **System Ports:** Hardware interactions are abstracted behind pure interfaces (`crates/system-ports`). 
- **Frictionless Portability:** Porting this OTA system to an STM32, ESP32, or nRF52 requires zero changes to the core logic. It only requires implementing a new `platform-<mcu>` crate to satisfy the system ports.
  
## Engineering Approach

Development is organized around architecture, implementation, and verification.

1. **Requirements and constraints:** identify memory, flash, boot, security, and recovery constraints imposed by the RP2350 platform.
2. **System architecture:** define component responsibilities, boot sequence, image lifecycle, and persistent state.
3. **Architecture Decision Records:** document design alternatives, trade-offs, and selected approaches.
4. **Implementation and verification:** implement individual mechanisms and validate their behavior on hardware.

Architecture decisions are maintained in:

```text
docs/adr/
```

## Update Lifecycle

The following diagram describes the intended update sequence, from image reception to boot confirmation or recovery.

```mermaid
flowchart TD
    A["Update request"] --> B["Receive firmware image"]
    B --> C["Write to inactive image slot"]
    C --> D["Verify image integrity"]
    D --> E["Verify image signature"]

    D -->|Invalid| F["Reject candidate"]
    E -->|Invalid| F

    F --> G["Retain confirmed image"]

    E -->|Valid| H["Record candidate in boot metadata"]
    H --> I["Restart device"]
    I --> J["Boot candidate image"]
    J --> K{"Application confirms successful startup?"}

    K -->|Yes| L["Mark candidate as confirmed"]
    K -->|No| M["Recover previous confirmed image"]

    L --> N["Normal operation"]
    M --> N
    G --> N
```

Image integrity and authenticity are treated as separate verification steps. Integrity checking detects unintended modifications, while signature verification establishes whether the image was authorized by a trusted signing key.

The boot metadata is part of the update mechanism: it must allow the boot stage to distinguish a confirmed image from an image undergoing trial execution.

## Boot Architecture

The boot sequence is based on the RP2350 startup process and the bootloader responsibilities defined by the project.

```mermaid
flowchart TD
    A["RP2350 Boot ROM"]
        --> B["Stage 2 Bootloader"]
        --> C["Boot Metadata Evaluation"]

    C --> D{"Candidate Image Pending?"}

    D -->|"Yes"| E["Select Candidate Image"]
    D -->|"No"| F["Select Confirmed Image"]

    E --> G["Firmware Execution"]
    F --> G

```

The update path and boot path have different responsibilities.

The update path receives and stores a candidate image, performs the required validation, and updates persistent boot metadata. The boot path evaluates that metadata and determines which image is eligible for execution.

This separation is intended to keep update transport and image delivery independent from boot selection.

## Planned Capabilities

- Dual-image storage and candidate image management.
- Persistent boot metadata and image state tracking.
- SHA-256 image digest verification.
- Digital signature verification.
- Trial boot and application confirmation.
- Recovery to the previously confirmed image.
- WiFi-based firmware delivery.
- Use of the RP2350 hardware SHA-256 accelerator.
- Evaluation of OTP-based rollback protection.

## Platform and Toolchain

| Component | Technology |
|---|---|
| MCU | RP2350 |
| Development board | Raspberry Pi Pico 2W |
| Language | Rust (`no_std`) |
| Async runtime | Embassy |
| Logging | `defmt` / RTT |
| Build system | Cargo |

## Repository Structure

The workspace isolates binary entry points (`firmware/`) from the hardware-agnostic logic (`crates/`). 

<details>
<summary><b>📂 Click to expand the full architectural workspace tree</b></summary>

```text
rp2350-secure-ota/
├── Cargo.toml
├── .cargo/
│   └── config.toml
│
├── firmware/             # Executable binary entry points
│   ├── application/      # Active slot application (main.rs, memory.x)
│   └── bootloader/       # Secure bootloader (main.rs, memory.x)
│
├── crates/               # Hardware-agnostic core logic
│   ├── update-core/      # OTA session, download, and agent logic
│   ├── boot-core/        # Image selection, validation, and recovery
│   ├── boot-contracts/   # Shared memory offsets, structs, and state flags
│   ├── system-ports/     # Abstract traits (Flash, Crypto, Reset)
│   ├── platform-rp2350/  # RP2350-specific HAL implementation of system-ports
│   └── transport/        # Network and serial interfaces (WiFi, UART)
│
├── tools/                # Host-side utilities
│   └── firmware-signer/  # CLI tool for hashing, signing, and packaging OTA images
│
├── tests/                # Unit and integration tests (Host & Hardware)
└── docs/                 # Architecture Decision Records (ADRs) and specs
```
</details>

## Development Status

**Current phase: architecture and platform investigation.**

The initial work is focused on:

- Understanding the RP2350 boot process and available boot mechanisms.
- Establishing the application and bootloader memory layout.
- Evaluating flash storage constraints for an A/B image arrangement.
- Defining the persistent metadata required for image selection and recovery.
- Investigating the available hardware support for cryptographic operations.

The complete OTA workflow is not yet implemented. The architecture and its recovery guarantees will be refined as platform constraints are verified.

## Future Development

The next development steps include:

- Finalizing the image storage and boot metadata contracts.
- Implementing the image verification pipeline.
- Developing the firmware signing toolchain.
- Integrating WiFi-based image transfer.
- Implementing trial boot, confirmation, and rollback.
- Testing interrupted flash operations and power-loss recovery.
- Evaluating OTP-based anti-rollback mechanisms.

## License

MIT / Apache-2.0 (to be determined).
