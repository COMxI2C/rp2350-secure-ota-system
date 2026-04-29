# ADR-001: Boot Strategy using RP2350 Secure Boot and Stage 2 OTA Bootloader

## Status

Accepted

---

## Context

The RP2350 provides a built-in Boot ROM (Stage 1) that supports secure boot by verifying firmware authenticity using cryptographic signatures (ECDSA P-256 with SHA-256).

The goal of this project is to design a **secure and resilient OTA firmware update system**, ensuring:

* The device never becomes unusable (no bricking)
* Firmware authenticity and integrity are guaranteed
* Updates are robust against power loss
* Rollback is possible using dual firmware slots (A/B)

A key architectural decision is defining **how the boot process is structured**, and how responsibilities are divided between hardware and software.

---

## Decision

We adopt a **multi-stage boot architecture**:

### Stage 1 — Boot ROM (Hardware Root of Trust)

* Responsible for verifying the authenticity of the Stage 2 bootloader
* Uses built-in cryptographic verification (ECDSA + SHA-256)
* Acts as the immutable root of trust
* No custom logic is implemented at this stage

---

### Stage 2 — Custom Bootloader (This Project)

* Verified and executed by the Boot ROM
* Responsible for:

  * Reading persistent metadata from flash
  * Determining the active firmware slot (A/B)
  * Managing OTA update flow
  * Verifying downloaded firmware images
  * Handling rollback and recovery
  * Configuring flash mapping (XIP aliasing)

---

### Application Firmware (Slot A / Slot B)

* Runs at a fixed virtual address
* Selected dynamically via metadata and XIP mapping
* Must be cryptographically verified before execution

---

## Architectural Flow

```text
Boot ROM (Stage 1)
        ↓
Verify Stage 2 Bootloader
        ↓
Execute Stage 2 Bootloader
        ↓
Read metadata (flash)
        ↓
Select firmware slot (A/B)
        ↓
Configure XIP mapping
        ↓
Jump to application firmware
```

---

## Alternatives Considered

### 1. Full Custom Bootloader (No Secure Boot)

* Pros:

  * Full control over boot process
* Cons:

  * No hardware root of trust
  * Increased complexity and security risk

→ Rejected due to lack of hardware-backed security

---

### 2. Single-Stage Boot (No Stage 2)

* Pros:

  * Simpler architecture
* Cons:

  * No OTA flexibility
  * No rollback support
  * No update lifecycle control

→ Rejected due to insufficient functionality

---

### 3. Software-Only Verification

* Pros:

  * Full flexibility
* Cons:

  * Slower
  * Less secure than hardware-backed validation

→ Rejected in favor of hardware-assisted security

---

## Consequences

### Positive

* Leverages RP2350 hardware security features
* Establishes a strong root of trust
* Enables robust OTA workflows
* Supports rollback and fault recovery
* Clear separation of responsibilities

---

### Negative / Trade-offs

* Requires understanding Boot ROM behavior
* Adds complexity with multi-stage architecture
* Requires additional metadata management
* Firmware verification must still be implemented in Stage 2 for OTA updates

---

## Security Considerations

* The Boot ROM guarantees authenticity of the bootloader
* OTA firmware must be verified independently before activation
* Metadata integrity is critical and must be protected against corruption
* Anti-rollback mechanisms will be implemented using OTP memory

---

## Notes

* XIP address aliasing will be used to allow fixed firmware execution addresses across A/B slots
* Metadata design (state, versioning, flags) will be defined in a separate ADR
* OTA communication and protocol design will be addressed in future ADRs
