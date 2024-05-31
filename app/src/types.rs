#![allow(unused_imports)]
use serde::Deserialize;
use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Debug,Deserialize)]
pub struct 
Source 
{
    pub name: String,
    pub path: String
}

/* Use ipv4 for now. In the future, detect the type of address. */
#[derive(Debug,Deserialize)]
pub struct 
Destination 
{
    pub address: Ipv4Addr,
    pub port: u16
}

#[derive(Debug,Deserialize)]
pub
struct 
Log 
{
    pub source: Source,
    pub destination: Destination,
    pub compression_level: Option<u8>,
    pub key: Option<String>,
    pub tx_buffer: Option<String>,
}

impl Log {
    pub fn get_tx_buffer(&self) -> usize {
        let mut bsize = 0;
        let val = self.tx_buffer.clone().unwrap_or("stream".to_string());
        match val.as_str() {
            "1kb" => bsize = 1024,
            "4kb" => bsize = 4096,
            "1mb" => bsize = 1024 * 1024,
            _ => bsize = 0, // stream the data
        }
        bsize
    }
    pub fn get_source_path(&self) -> String {
        self.source.path.to_string()
    }
    pub fn get_destination_address(&self) -> Ipv4Addr {
        self.destination.address
    } 
}

// fallback values
#[derive(Debug, Deserialize)]
pub
struct 
Defaults 
{
    pub compression_level: u8,
    pub key: String,
    pub tx_buffer: String,
}

#[derive(Debug, Deserialize)]
pub
struct 
Config 
{
    pub logs: Vec<Log>,
    pub defaults: Defaults,
}

#[derive(Debug, Serialize, Deserialize)]
pub
struct
Header
{
    pub name: String,
    pub date: String,
}
