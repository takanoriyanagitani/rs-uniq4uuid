use std::collections::BTreeSet;
use std::io;

pub fn uuids2bset<I>(uuids: I) -> Result<BTreeSet<u128>, io::Error>
where
    I: Iterator<Item = Result<u128, io::Error>>,
{
    uuids.collect()
}
