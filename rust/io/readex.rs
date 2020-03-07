use std::io::{Read, Error, ErrorKind, Result};
use std::io::Cursor;

trait ReadExt: Read {
    fn read_u8(&mut self) -> Result<u8> {
        // 示例代码，效率问题不在考虑之内
        let mut bytes = [0u8; 1];
        let size = try!(self.read(&mut bytes[..]));
        if size != 1 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "read not one bytes"));
        }
        Ok(bytes[0])
    }
}

impl<T: Read> ReadExt for T {}

fn main() {
    let mut cursor = Cursor::new(vec![0u8, 1, 2]);
    let value = cursor.read_u8().unwrap();
    println!("read u8: {}", value);
}
