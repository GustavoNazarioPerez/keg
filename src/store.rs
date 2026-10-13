// Scaffolding for now
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::time::SystemTime;

struct KeyDirEntry {
    crc: u32,
    file_id: u32,
    value_sz: u32,
    value_pos: u32,
    tstamp: SystemTime,
}

struct DataFile {
    f: File,
    offset: usize,
}

#[derive(Default)]
pub struct Store {
    keydir: HashMap<String, KeyDirEntry>,
    files: Vec<DataFile>,
}
