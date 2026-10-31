use droidlink_local_policy::{LocalEndpoint, PolicyError};

#[test]
fn accepts_private_ipv4_ranges() {
    for input in ["10.1.2.3:5555", "172.16.0.1:1", "192.168.1.20:65535"] {
        assert!(LocalEndpoint::parse(input).is_ok());
    }
}

#[test]
fn checks_172_range_boundaries() {
    assert!(LocalEndpoint::parse("172.16.0.1:80").is_ok());
    assert!(LocalEndpoint::parse("172.31.255.254:80").is_ok());
    for input in ["172.15.255.254:80", "172.32.0.1:80"] {
        assert_eq!(
            LocalEndpoint::parse(input),
            Err(PolicyError::AddressOutsideAllowlist)
        );
    }
}

#[test]
fn accepts_ipv6_ula() {
    for input in ["[fc00::1]:5555", "[fdff::1234]:65535"] {
        assert!(LocalEndpoint::parse(input).is_ok());
    }
}

#[test]
fn rejects_public_addresses() {
    for input in ["8.8.8.8:443", "[2001:4860:4860::8888]:443"] {
        assert_eq!(
            LocalEndpoint::parse(input),
            Err(PolicyError::AddressOutsideAllowlist)
        );
    }
}

#[test]
fn rejects_loopback_unspecified_and_multicast() {
    for input in [
        "127.0.0.1:80",
        "0.0.0.0:80",
        "224.0.0.1:80",
        "[::1]:80",
        "[::]:80",
        "[ff02::1]:80",
    ] {
        assert!(LocalEndpoint::parse(input).is_err());
    }
}

#[test]
fn rejects_link_local_and_carrier_grade_nat() {
    for input in ["169.254.1.1:80", "100.64.0.1:80", "[fe80::1]:80"] {
        assert!(LocalEndpoint::parse(input).is_err());
    }
}

#[test]
fn rejects_ipv4_mapped_ipv6() {
    assert!(LocalEndpoint::parse("[::ffff:192.168.1.20]:5555").is_err());
}

#[test]
fn rejects_zero_and_out_of_range_ports() {
    assert_eq!(
        LocalEndpoint::parse("192.168.1.20:0"),
        Err(PolicyError::InvalidPort)
    );
    assert!(LocalEndpoint::parse("192.168.1.20:65536").is_err());
}

#[test]
fn rejects_names_urls_and_injected_arguments() {
    for input in [
        "phone.local:5555",
        "https://192.168.1.20:5555",
        "192.168.1.20:5555; whoami",
        "192.168.1.20:5555 --listen",
        "",
    ] {
        assert_eq!(
            LocalEndpoint::parse(input),
            Err(PolicyError::InvalidSocketAddress)
        );
    }
}

#[test]
fn rejects_surrounding_whitespace() {
    assert!(LocalEndpoint::parse(" 192.168.1.20:5555").is_err());
    assert!(LocalEndpoint::parse("192.168.1.20:5555\n").is_err());
}

#[test]
fn preserves_the_selected_endpoint() {
    let endpoint = LocalEndpoint::parse("192.168.1.20:5555").unwrap();
    assert_eq!(endpoint.socket_addr().to_string(), "192.168.1.20:5555");
}

#[test]
fn rejects_scoped_ula_addresses() {
    assert!(LocalEndpoint::parse("[fd00::1%3]:5555").is_err());
}
