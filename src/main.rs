use std::io;
use std::process::ExitCode;

use std::collections::BTreeSet;

use rs_uniq4uuid::reader2uuids::stdin2uuids;
use rs_uniq4uuid::set2writer::bset2stdout;
use rs_uniq4uuid::uuids2set::uuids2bset;

fn sub() -> Result<(), io::Error> {
    let uuids = stdin2uuids();
    let bset: BTreeSet<u128> = uuids2bset(uuids)?;
    bset2stdout(&bset)?;
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
