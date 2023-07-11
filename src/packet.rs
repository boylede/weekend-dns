use rand::Rng;
use std::fmt::Write;

use crate::deserialization::pop_u16;
use crate::domain_name::DomainName;
use crate::record::{Class, Kind};
use crate::serialization::push_u16;
use crate::record::Record;


#[derive(Debug)]
pub struct Packet {
    id: u16,
    flags: u16,
    questions: Vec<Question>,
    answers: Vec<Record>,
    authorities: Vec<Record>,
    additionals: Vec<Record>,
}

impl Packet {
    pub fn new() -> Packet {
        let id = rand::thread_rng().gen();
        let flags = 1 << 8;
        Packet {
            id,
            flags,
            questions: vec![],
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        }
    }
    pub fn with_question(mut self, question: Question) -> Packet {
        self.questions.push(question);
        self
    }
    pub fn build(domain: &str, kind: Kind) -> Packet {
        let q = Question::build(domain, kind);
        let id = rand::thread_rng().gen();
        let flags = 1 << 8;
        Packet {
            id,
            flags,
            questions: vec![q],
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        }
    }
    pub fn with_id(mut self, id: u16) -> Packet {
        self.id = id;
        self
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        push_u16(&mut buf, self.id);
        push_u16(&mut buf, self.flags);
        push_u16(&mut buf, self.questions.len() as u16);
        push_u16(&mut buf, 0);
        push_u16(&mut buf, 0);
        push_u16(&mut buf, 0);

        for question in self.questions.iter() {
            buf.extend_from_slice(&question.to_bytes());
        }
        buf
    }
    pub fn from_bytes(buf: &[u8]) -> Option<Packet> {
        let mut cursor = 0;
        let header = Header::from_bytes(buf, &mut cursor)?;

        println!("got response with header: {:?}", header);

        todo!();
    }
}

#[derive(Debug)]
pub struct Header {
    id: u16,
    flags: u16,
    questions: u16,
    answers: u16,
    authorities: u16,
    additionals: u16,
}

impl Header {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(12);
        push_u16(&mut buf, self.id);
        push_u16(&mut buf, self.flags);
        push_u16(&mut buf, self.questions);
        push_u16(&mut buf, self.answers);
        push_u16(&mut buf, self.authorities);
        push_u16(&mut buf, self.additionals);
        buf
    }
    pub fn from_bytes(buf: &[u8], cursor: &mut usize) -> Option<Header> {
        let id = pop_u16(buf, cursor)?;
        let flags = pop_u16(buf, cursor)?;
        let questions = pop_u16(buf, cursor)?;
        let answers = pop_u16(buf, cursor)?;
        let authorities = pop_u16(buf, cursor)?;
        let additionals = pop_u16(buf, cursor)?;
        Some(Header {
            id,
            flags,
            questions,
            answers,
            authorities,
            additionals,
        })
    }
}

#[derive(Debug)]
pub struct Question {
    name: DomainName,
    kind: Kind,
    class: Class,
}

impl Question {
    pub fn new() -> Question {
        Question {
            name: DomainName::empty(),
            kind: Kind::A,
            class: Class::Internet,
        }
    }
    pub fn with_domain_name(mut self, name: &str) -> Question {
        self.name = DomainName::new(name);
        self
    }
    pub fn build(name: &str, kind: Kind) -> Question {
        let name = DomainName::new(name);
        Question {
            name,
            kind,
            class: Class::Internet,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = self.name.to_bytes();
        push_u16(&mut buf, self.kind as u16);
        push_u16(&mut buf, self.class as u16);
        buf
    }
}
