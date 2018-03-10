#![feature(type_ascription)]
extern crate ammonia;
#[macro_use]
extern crate log;
extern crate number_prefix;
extern crate pretty_env_logger;
extern crate reqwest;
extern crate retry;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

#[macro_use]
extern crate maplit;

mod teledex;

pub use teledex::{DefinitionFetcher, DexBot, MockDefinitionFetcher};
