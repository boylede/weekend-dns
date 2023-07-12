use std::fmt::Display;

use crate::deserialization::{pop_collection, pop_u16, pop_u8, FromBytes};

#[derive(Debug, Clone)]
pub enum DomainName {
    Simple(String),
    Compressed(String, u16),
}

impl DomainName {
    pub fn new(name: &str) -> DomainName {
        DomainName::Simple(name.to_string())
    }
    pub fn empty() -> DomainName {
        DomainName::new("")
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let DomainName::Simple(inner) = self else {
            panic!("should decrompress before serializing");
        };
        let mut buf = Vec::with_capacity(inner.len());
        let parts = inner.split('.');
        for part in parts {
            let len = part.len();
            buf.push(len as u8);
            buf.extend_from_slice(part.as_bytes());
        }
        buf.push(0);
        buf
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainName::Simple(name) => <String as Display>::fmt(name, f),
            DomainName::Compressed(prev, loc) => {
                <String as Display>::fmt(&format!("Cmp\"{prev}\"+{loc}"), f)
            }
        }
    }
}

impl FromBytes for DomainName {
    fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Self> {
        let mut parts = Vec::new();
        const MAX_LOOPS: usize = 256;
        let mut counter: usize = 0;
        loop {
            counter += 1;
            if counter >= MAX_LOOPS {
                return None;
            }
            let len = pop_u8(buf, cursor)? as u16;
            if len == 0 {
                break;
            } else if (len & 0b11000000) == 0b11000000 {
                let lo = pop_u8(buf, cursor)? as u16;
                let hi = (len & 0b00111111) << 8;
                let pointer = (hi | lo);
                return Some(DomainName::Compressed(parts.join("."), pointer));
            }
            let string: String = pop_collection::<char>(buf, cursor, len as usize)?
                .iter()
                .collect();
            parts.push(string);
        }
        Some(DomainName::Simple(parts.join(".")))
    }
}
