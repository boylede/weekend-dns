use crate::domain_name::DomainName;

#[derive(Debug, Clone)]
pub struct Record {
    name: DomainName,
    kind: Kind,
    class: Class,
    ttl: i32,
    data: Content,
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

#[derive(Debug, Copy, Clone)]
pub enum Class {
    Internet = 1,
}
