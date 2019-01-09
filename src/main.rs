#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(clippy::stutter)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

extern crate systemstat;

use crate::proto::*;
use systemstat::Platform;
use std::io::Result as IoResult;

pub mod proto;

struct System {
    platform: systemstat::System,
}

impl System {
    pub fn new() -> Self {
        let platform = systemstat::System::new();
        Self { platform }
    }

    pub fn available_memory(&self) -> IoResult<usize> {
        self.platform.memory().map(|mem| mem.free.as_usize() / 1024)
    }

    pub fn check_resource(&self, res: &Resource) -> IoResult<bool> {
        match res {
            Resource::Memory(MemoryReq::Absolute(free)) => Ok(&self.available_memory()? >= free),
            _ => Ok(false)
        }
    }
}

fn main() {
    println!("Hello, world!");

    let sys = System::new();
    println!("Memory available: {:?} KiB", sys.available_memory().unwrap());

    let memcon1 = proto::Constraint::required(proto::Resource::Memory(proto::MemoryReq::Absolute(10240)));
    println!("\n{:?}\nPasses: {}", memcon1, sys.check_resource(&memcon1.resource).unwrap());

    let memcon2 = proto::Constraint::required(proto::Resource::Memory(proto::MemoryReq::Absolute(102400000)));
    println!("\n{:?}\nPasses: {}", memcon2, sys.check_resource(&memcon2.resource).unwrap());
}
