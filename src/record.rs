use std::fmt::Display;

use crate::{domain_name::DomainName, deserialization::{FromBytes, pop_u16, pop_collection}};

#[derive(Debug, Clone)]
pub struct Record {
    name: DomainName,
    kind: Kind,
    class: Class,
    ttl: i32,
    data: Vec<u8>,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <DomainName as Display>::fmt(&self.name, f)?;
        write!(f, " ")?;
        <Kind as Display>::fmt(&self.kind, f)?;
        write!(f, " ")?;
        <Class as Display>::fmt(&self.class, f)?;
        write!(f, " {} ", self.ttl)?;
        use Kind::*;
        match self.kind {
            A => {
                for byte in self.data.iter() {
                    write!(f, "{byte:#x} ")?;
                }
            },
            NS => todo!(),
            MD => todo!(),
            MF => todo!(),
            CNAME => {
                match DomainName::from_bytes(&self.data, &mut 0) {
                    Some(canonical) => {
                        write!(f, "{}", canonical)?;
                    }
                    None => {
                        for byte in self.data.iter() {
                            write!(f, "{byte:#x}")?;
                        }
                    }
                };
            },
            SOA => todo!(),
            MB => todo!(),
            MG => todo!(),
            MR => todo!(),
            NULL => todo!(),
            WKS => todo!(),
            PTR => todo!(),
            HINFO => todo!(),
            MINFO => todo!(),
            MX => todo!(),
            TXT => todo!(),
        }
        Ok(())
    }
}

impl FromBytes for Record {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Self> {
        let name = DomainName::from_bytes(buf, cursor)?;
        let kind = Kind::from_bytes(buf, cursor)?;
        let class = Class::from_bytes(buf, cursor)?;
        let ttl = i32::from_bytes(buf, cursor)?;
        let count = pop_u16(buf, cursor)?;
        let data = pop_collection(buf, cursor, count as usize)?;
        Some(Record { name, kind, class, ttl, data })
    }
}


#[derive(Debug, Clone)]
pub enum Content {
    IPv4,
    IPv6,
    DomainName(DomainName),
    DomainNameRef(usize),
}

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    /// a host address
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
            _ => Err(())
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s= match self {
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

#[derive(Debug, Copy, Clone)]
pub enum Class {
    Internet = 1,
}
impl TryFrom<u16> for Class {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use Class::*;
        match value {
            1 => Ok(Internet),
            _ => Err(())
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s= match self {
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