#![forbid(unsafe_code)]

use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};

use ironsea_store::{Store, Load};

pub fn load<T: Load>(from: String) -> io::Result<T> {
    let file_in = File::open(from)?;
    let reader = BufReader::new(&file_in);

    T::load(reader)
}

pub fn store<T: Store>(table: &mut T, to: String) -> io::Result<()> {
    let file_out = File::create(to)?;
    let writer = BufWriter::new(&file_out);

    table.store(writer)
}
