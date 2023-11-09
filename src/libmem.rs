use std::io::{Error, ErrorKind};

pub(crate) fn memw(src: &[u8], dest: &mut [u8], addr: usize) -> Result<(), Error> {
    if addr + src.len() > dest.len() {
        return Err(Error::new(
            ErrorKind::OutOfMemory,
            "too many data, out of bounds",
        ));
    }
    dest[addr..addr + src.len()].copy_from_slice(src);
    Ok(())
}

pub(crate) fn memr(src: &[u8], addr: usize, amount: usize) -> Result<&[u8], Error> {
    if addr + amount > src.len() {
        return Err(Error::new(ErrorKind::OutOfMemory, "reading out of bounds"));
    }
    Ok(&src[addr..addr + amount])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memw() {
        let mut memory = [0u8; 1024];
        let data = "hello world!";
        memw(data.as_bytes(), &mut memory, 0x0).unwrap();
        assert_eq!(data.as_bytes(), &memory[0..data.len()])
    }

    #[test]
    fn test_memr() {
        let mut memory = [0u8; 1024];
        let data = "hello_world!";
        memw(data.as_bytes(), &mut memory, 0x0).unwrap();
        let read = memr(&memory, 0x0, data.len()).unwrap();
        assert_eq!(data.as_bytes(), read);
    }
}
