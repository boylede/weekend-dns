use std::{
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
};

use crate::{
    deserialization::{pop_collection, pop_u16, FromBytes},
    domain_name::DomainName,
};

#[derive(Debug, Clone)]
pub struct Record {
    name: DomainName,
    kind: Kind,
    class: Class,
    ttl: i32,
    data: Content,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <DomainName as Display>::fmt(&self.name, f)?;
        write!(f, " ")?;
        <Kind as Display>::fmt(&self.kind, f)?;
        write!(f, " ")?;
        <Class as Display>::fmt(&self.class, f)?;
        write!(f, " {} ", self.ttl)?;
        write!(f, "{}", self.data)
    }
}

impl FromBytes for Record {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Self> {
        let name = DomainName::from_bytes(buf, cursor)?;
        let kind = Kind::from_bytes(buf, cursor)?;
        let class = Class::from_bytes(buf, cursor)?;
        let ttl = i32::from_bytes(buf, cursor)?;
        let count = pop_u16(buf, cursor)?;
        use Kind::*;
        let data = match kind {
            A => {
                if let Some(ip6) = <Ipv6Addr as FromBytes>::from_bytes(buf, cursor) {
                    Content::IPv6(ip6)
                } else {
                    let ip = <Ipv4Addr as FromBytes>::from_bytes(buf, cursor)?;
                    Content::IPv4(ip)
                }
            }
            // NS => todo!(),
            // MD => todo!(),
            // MF => todo!(),
            CNAME => {
                let domain = <DomainName as FromBytes>::from_bytes(buf, cursor)?;
                Content::DomainName(domain)
            }
            SOA => {
                let domain = <DomainName as FromBytes>::from_bytes(buf, cursor)?;
                Content::DomainName(domain)
            }
            // MB => todo!(),
            // MG => todo!(),
            // MR => todo!(),
            // NULL => todo!(),
            // WKS => todo!(),
            // PTR => todo!(),
            // HINFO => todo!(),
            // MINFO => todo!(),
            // MX => todo!(),
            // TXT => todo!(),
            _ => {
                let data = pop_collection(buf, cursor, count as usize)?;
                Content::Other(data)
            }
        };
        Some(Record {
            name,
            kind,
            class,
            ttl,
            data,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Content {
    IPv4(Ipv4Addr),
    IPv6(Ipv6Addr),
    DomainName(DomainName),
    Other(Vec<u8>),
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::IPv4(ip) => write!(f, "{ip}"),
            Content::IPv6(ip) => write!(f, "{ip}"),
            Content::DomainName(dn) => write!(f, "{dn}"),
            Content::Other(bytes) => {
                for byte in bytes.iter() {
                    write!(f, "{byte:02x} ")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Kind {
    /// a host address
    #[default]
    A = 1,
    /// an authoritative name server
    NS = 2,
    /// a mail destination (Obsolete - use MX)     
    MD = 3,
    /// a mail forwarder (Obsolete - use MX)
    MF = 4,
    /// the canonical name for an alias
    CNAME = 5,
    /// marks the start of a zone of authority
    SOA = 6,
    /// a mailbox domain name (EXPERIMENTAL)
    MB = 7,
    /// a mail group member (EXPERIMENTAL)
    MG = 8,
    /// a mail rename domain name (EXPERIMENTAL)
    MR = 9,
    /// a null RR (EXPERIMENTAL)
    NULL = 10,
    /// a well known service description
    WKS = 11,
    /// a domain name pointer
    PTR = 12,
    /// host information
    HINFO = 13,
    /// mailbox or mail list information
    MINFO = 14,
    /// mail exchange
    MX = 15,
    /// text strings
    TXT = 16,
}

impl TryFrom<u16> for Kind {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use Kind::*;
        match value {
            1 => Ok(A),
            2 => Ok(NS),
            3 => Ok(MD),
            4 => Ok(MF),
            5 => Ok(CNAME),
            6 => Ok(SOA),
            7 => Ok(MB),
            8 => Ok(MG),
            9 => Ok(MR),
            10 => Ok(NULL),
            11 => Ok(WKS),
            12 => Ok(PTR),
            13 => Ok(HINFO),
            14 => Ok(MINFO),
            15 => Ok(MX),
            16 => Ok(TXT),
            _ => Err(()),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Kind::A => "A",
            Kind::NS => "NS",
            Kind::MD => "MD",
            Kind::MF => "MF",
            Kind::CNAME => "CNAME",
            Kind::SOA => "SOA",
            Kind::MB => "MB",
            Kind::MG => "MG",
            Kind::MR => "MR",
            Kind::NULL => "NULL",
            Kind::WKS => "WKS",
            Kind::PTR => "PTR",
            Kind::HINFO => "HINFO",
            Kind::MINFO => "MINFO",
            Kind::MX => "MX",
            Kind::TXT => "TXT",
        };
        write!(f, "{s}")
    }
}

impl FromBytes for Kind {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Kind> {
        let num = pop_u16(buf, cursor)?;
        num.try_into().ok()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Class {
    #[default]
    Internet = 1,
}
impl TryFrom<u16> for Class {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use Class::*;
        match value {
            1 => Ok(Internet),
            _ => Err(()),
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Class::Internet => "IN",
        };
        write!(f, "{s}")
    }
}

impl FromBytes for Class {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Class> {
        let num = pop_u16(buf, cursor)?;
        num.try_into().ok()
    }
}
