use crate::domain_name::DomainName;
use crate::record::Class;
use crate::record::Kind;

pub struct Record {
    name: DomainName,
    kind: Kind,
    class: Class,
    ttl: i32,
    data: Content,
}

pub enum Content {
    IPv4,
    IPv6,
    DomainName(DomainName),
}
