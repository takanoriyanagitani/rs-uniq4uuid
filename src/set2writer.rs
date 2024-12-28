use std::collections::BTreeSet;
use std::io;
use std::io::BufWriter;
use std::io::Write;

pub fn bset2writer<W>(bset: &BTreeSet<u128>, mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let mut bw = BufWriter::new(&mut wtr);
    for id in bset {
        let a: [u8; 16] = id.to_be_bytes();
        bw.write_all(&a[..])?;
    }
    drop(bw);
    wtr.flush()?;
    Ok(())
}

pub fn bset2stdout(bset: &BTreeSet<u128>) -> Result<(), io::Error> {
    let o = io::stdout();
    let ol = o.lock();
    bset2writer(bset, ol)
}
