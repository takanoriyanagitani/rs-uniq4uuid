use std::io;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Read;

pub struct UuidsReader<R> {
    original: R,
}

impl<R> Iterator for UuidsReader<R>
where
    R: Read,
{
    type Item = Result<u128, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: [u8; 16] = [0; 16];
        let ms: &mut [u8] = &mut buf[..];
        let res: Result<u128, io::Error> = self
            .original
            .read_exact(ms)
            .map(|_| u128::from_be_bytes(buf));
        match res {
            Ok(u) => Some(Ok(u)),
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => None,
                _ => Some(Err(e)),
            },
        }
    }
}

pub fn reader2uuids<R>(rdr: R) -> impl Iterator<Item = Result<u128, io::Error>>
where
    R: Read,
{
    let original = BufReader::new(rdr);
    UuidsReader { original }
}

pub fn stdin2uuids() -> impl Iterator<Item = Result<u128, io::Error>> {
    let i = io::stdin();
    let il = i.lock();
    reader2uuids(il)
}
