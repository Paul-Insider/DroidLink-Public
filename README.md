# DROIDLINK · Public Engineering

**Rust engineering. Direct device connections. A local-first product direction.**

DROIDLINK is a proprietary desktop project focused on connecting mobile devices
to computers. This public repository presents a deliberately small, inspectable
engineering sample and an explicit trust boundary.

## What you can inspect

| Item | Purpose |
| --- | --- |
| `src/lib.rs` | A real, dependency-free Rust library validating a private-address allowlist |
| `src/main.rs` | A CLI demonstrating acceptance and rejection without making connections |
| `tests/policy.rs` | 12 integration tests covering ranges, malformed input and rejection cases |
| `docs/trust-boundary.md` | What this sample demonstrates, and what it does not establish |
| `LICENSE` and `legal/` | Public evaluation terms, ownership and third-party boundaries |

This sample was written specifically for public inspection. It is **not extracted
from the private engine** and is **not claimed to be integrated into a released
DROIDLINK build**. It does not implement mirroring, device control, encryption,
authentication or licensing.

## Run locally

Requires an installed Rust toolchain (Rust 2021 edition). No external Cargo
dependencies are required; these commands run offline once Rust is installed.

```bash
cargo test --offline --locked
cargo run --offline --locked -- 192.168.1.20:5555
cargo run --offline --locked -- 8.8.8.8:443
```

The first example exits successfully. The second is deliberately rejected with
exit code 1. A missing or extra CLI argument exits with code 2.

Allowed addresses: RFC1918 IPv4 and IPv6 ULA with a nonzero port.
DNS names, URLs, loopback, public, multicast, link-local, scoped IPv6 and
IPv4-mapped IPv6 addresses are rejected by this demonstration's policy.

**Private address space does not prove a device is on the same physical LAN.**
VPNs and routing can cross network boundaries. This parser is not a firewall,
a permission system, a reachability check or proof of an application's No Cloud claim.

## Product direction and evidence

DROIDLINK's intended direction is direct USB/local Wi-Fi device communication,
without a cloud relay for session contents. Cross-platform compatibility,
transport security and release readiness require testing of actual product builds.
This repository makes no claim of security certification, completed iPhone support,
or general production readiness.

## Rust on GitHub

Rust is the programming language of this repository's source files. GitHub
Linguist computes language statistics after publication to the default branch.
There are no language-spoofing overrides or generated filler files.

## Rights and contact

Created by **Patrice Paul SARDA — Paul Insider**.

The source is public for inspection and local evaluation under [LICENSE](LICENSE).
This is not an OSI-approved open-source license and does not license the private
DROIDLINK product. Commercial reuse requires written permission.

Public questions: use this repository's Issues tab after publication.
Do not post credentials, private logs or vulnerability details in public issues.
See [Legal information](legal/README.md), [SECURITY.md](SECURITY.md) and
[third-party notices](legal/third-party.md).
