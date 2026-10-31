//! Standalone public demonstration, not the private DROIDLINK connection engine.
//!
//! Accepts literal RFC1918 IPv4 or IPv6 ULA socket addresses with a nonzero port.
//! Does not resolve DNS, inspect routes, open sockets, or prove physical proximity.
//! A VPN can route private addresses. This policy is not a firewall or authentication.

#![forbid(unsafe_code)]

use std::fmt;
use std::net::{IpAddr, SocketAddr};

/// An address accepted by this demonstration's explicit allowlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalEndpoint(SocketAddr);

/// Rejections do not include the user's input, avoiding accidental log disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyError {
    InvalidSocketAddress,
    InvalidPort,
    ScopedAddressUnsupported,
    AddressOutsideAllowlist,
}

impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidSocketAddress => "expected a literal IP address and port",
            Self::InvalidPort => "port zero is not permitted",
            Self::ScopedAddressUnsupported => "scoped IPv6 addresses are not supported",
            Self::AddressOutsideAllowlist => "address is outside the private-address allowlist",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for PolicyError {}

impl LocalEndpoint {
    /// Parse without DNS lookup or any network operation.
    ///
    /// ```
    /// use droidlink_local_policy::LocalEndpoint;
    /// assert!(LocalEndpoint::parse("192.168.1.20:5555").is_ok());
    /// assert!(LocalEndpoint::parse("8.8.8.8:443").is_err());
    /// assert!(LocalEndpoint::parse("example.com:443").is_err());
    /// ```
    pub fn parse(input: &str) -> Result<Self, PolicyError> {
        let address: SocketAddr = input
            .parse()
            .map_err(|_| PolicyError::InvalidSocketAddress)?;
        if address.port() == 0 {
            return Err(PolicyError::InvalidPort);
        }
        if let SocketAddr::V6(v6) = address {
            if v6.scope_id() != 0 {
                return Err(PolicyError::ScopedAddressUnsupported);
            }
        }
        let allowed = match address.ip() {
            IpAddr::V4(ip) => {
                let octets = ip.octets();
                octets[0] == 10
                    || (octets[0] == 172 && (16..=31).contains(&octets[1]))
                    || (octets[0] == 192 && octets[1] == 168)
            }
            IpAddr::V6(ip) => ip.segments()[0] & 0xfe00 == 0xfc00,
        };
        if !allowed {
            return Err(PolicyError::AddressOutsideAllowlist);
        }
        Ok(Self(address))
    }

    /// Inspect the accepted value. No connection is established.
    pub fn socket_addr(self) -> SocketAddr {
        self.0
    }
}
