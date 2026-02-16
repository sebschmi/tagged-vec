//! Functions providing plain binary I/O for `TaggedVec`.
use std::{io::Read, marker::PhantomData, mem};

use crate::TaggedVec;

impl<Index, Value: Copy> TaggedVec<Index, Value> {
    /// Read a `TaggedVec` from the given reader by simply copying the bytes into the underlying vector.
    ///
    /// This is as if the reader was memory-mapped into the vector.
    /// For correct function, the bytes must have been written by [`Self::write_binary`] on a machine with the same pointer size and the same endianness.
    pub fn read_binary(mut reader: impl Read) -> std::io::Result<Self> {
        let mut buffer = [0; mem::size_of::<usize>()];
        reader.read_exact(&mut buffer)?;
        let len = usize::from_ne_bytes(buffer);

        let value_size = mem::size_of::<Value>();
        let data_bytes_len = value_size * len;

        let mut data = Vec::<Value>::with_capacity(data_bytes_len);
        let mut data_bytes = unsafe {
            Vec::from_raw_parts(
                data.as_mut_ptr() as *mut u8,
                0,
                data.capacity() * value_size,
            )
        };
        reader
            .by_ref()
            .take(data_bytes_len.try_into().unwrap())
            .read_to_end(&mut data_bytes)?;
        unsafe {
            data.set_len(len);
        };
        data_bytes.leak();

        Ok(Self {
            index_type: PhantomData,
            vec: data,
        })
    }

    /// Write a `TaggedVec` into the given writer by simply copying the bytes from the underlying vector.
    ///
    /// Note that this simple data format is as if the vector was memory-mapped into the writer.
    /// The data format is dependent on the machine's pointer size and endianness, so be careful when writing and reading on different machines.
    pub fn write_binary(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        writer.write_all(&self.len().to_ne_bytes())?;

        let value_size = mem::size_of::<Value>();
        let data_bytes_len = value_size * self.len();
        let data: &[u8] =
            unsafe { std::slice::from_raw_parts(self.vec.as_ptr() as *const u8, data_bytes_len) };
        writer.write_all(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::TaggedVec;

    #[test]
    fn test_binary_io() {
        let mut vec = TaggedVec::<usize, u64>::new();
        vec.push(42);
        vec.push(1337);

        let mut buffer = Vec::new();
        vec.write_binary(&mut buffer).unwrap();

        let read_vec = TaggedVec::<usize, u64>::read_binary(buffer.as_slice()).unwrap();
        assert_eq!(read_vec.as_untagged_slice(), &[42, 1337]);
    }
}
