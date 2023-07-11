#[derive(Debug, Clone)]
pub struct DomainName {
    inner: String,
}

impl DomainName {
    pub fn new(name: &str) -> DomainName {
        DomainName { inner: name.to_string() }
    }
    pub fn empty() -> DomainName {
        DomainName::new("")
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.inner.len());
        let parts = self.inner.split('.');
        for part in parts {
            let len = part.len();
            buf.push(len as u8);
            buf.extend_from_slice(part.as_bytes());
        }
        buf.push(0);
        buf
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
}
