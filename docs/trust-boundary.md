# Trust boundary and verification

## Demonstrated by this sample

- Explicit allowlisting instead of accepting arbitrary hostnames or URLs.
- No DNS resolution, networking, filesystem writes or subprocesses in the library.
- Error messages do not echo endpoint input.
- A typed accepted endpoint, private construction and explicit errors.
- Dependency-free Rust source with unsafe code forbidden.

The CLI reads one command-line argument and writes a result to stdout/stderr.
It does not establish a connection or control a device.

## Not demonstrated

This source does not establish the security, privacy or functionality of any
private DROIDLINK build. In particular it does not prove same-LAN membership,
authentication, consent, encryption, routing restrictions, safe ADB configuration,
revocation, safe clipboard handling, secure updates or absence of product telemetry.
It is not a general SSRF defense, complete firewall or security certification.
RFC1918/ULA addresses may be routed through VPNs. Some private addresses represent
network or broadcast destinations depending on the subnet; reachability is not checked.

## Product verification targets

Before making a product-level No Cloud claim, validate actual release binaries:
session traffic destinations; behavior with Internet access blocked; peer trust;
revocation; permissions; update integrity; diagnostic redaction; license renewal
behavior. Separate website/payment/download services from session data transport.

These are verification targets, not completed audit results.
