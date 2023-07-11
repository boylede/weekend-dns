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
}

#[derive(Debug, Copy, Clone)]
pub enum Kind {
    A = 1,      // a host address
    NS = 2,     // an authoritative name server
    MD = 3,     // a mail destination (Obsolete - use MX)
    MF = 4,     // a mail forwarder (Obsolete - use MX)
    CNAME = 5,  // the canonical name for an alias
    SOA = 6,    // marks the start of a zone of authority
    MB = 7,     // a mailbox domain name (EXPERIMENTAL)
    MG = 8,     // a mail group member (EXPERIMENTAL)
    MR = 9,     // a mail rename domain name (EXPERIMENTAL)
    NULL = 10,  // a null RR (EXPERIMENTAL)
    WKS = 11,   // a well known service description
    PTR = 12,   // a domain name pointer
    HINFO = 13, // host information
    MINFO = 14, // mailbox or mail list information
    MX = 15,    // mail exchange
    TXT = 16,   // text strings
}

#[derive(Debug, Copy, Clone)]
pub enum Class {
    Internet = 1,
}
