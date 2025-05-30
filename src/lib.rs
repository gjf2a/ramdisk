#![cfg_attr(not(test), no_std)]
use thiserror_no_std::Error;
use core::result::Result;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Error)]
pub enum RamDiskError {
    #[error("Attempt to read block {0}; maximum block {1}")]
    IllegalBlockRead(usize, usize),
    #[error("Attempt to write block {0}; maximum block {1}")]
    IllegalBlockWrite(usize, usize)
}

impl core::error::Error for RamDiskError {}

#[derive(core::fmt::Debug, Copy, Clone)]
pub struct RamDisk<const BLOCK_SIZE: usize, const NUM_BLOCKS: usize> {
    blocks: [[u8; BLOCK_SIZE]; NUM_BLOCKS],
}

impl<const BLOCK_SIZE: usize, const NUM_BLOCKS: usize> RamDisk<BLOCK_SIZE, NUM_BLOCKS> {
    pub fn new() -> Self {
        Self {
            blocks: [[0; BLOCK_SIZE]; NUM_BLOCKS],
        }
    }

    pub const fn blocks(blocks: [[u8; BLOCK_SIZE]; NUM_BLOCKS]) -> Self {
        Self {blocks}
    }

    pub fn num_blocks(&self) -> usize {
        NUM_BLOCKS
    }

    pub fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    pub fn total_storage(&self) -> usize {
        NUM_BLOCKS * BLOCK_SIZE
    }

    pub fn read(&self, block: usize, buffer: &mut [u8; BLOCK_SIZE]) -> Result<(), RamDiskError> {
        match self.blocks.get(block) {
            Some(found) => {
                *buffer = *found;
                Ok(())
            }
            None => Err(RamDiskError::IllegalBlockRead(block, self.num_blocks() - 1)),
        }
    }

    pub fn write(&mut self, block: usize, buffer: &[u8; BLOCK_SIZE]) -> Result<(), RamDiskError> {
        match self.blocks.get_mut(block) {
            Some(found) => {
                *found = *buffer;
                Ok(())
            }
            None => Err(RamDiskError::IllegalBlockWrite(block, self.num_blocks() - 1)),
        }
    }

    pub fn write_from_str(&mut self, block: usize, contents: &str) -> Result<(), RamDiskError> {
        match self.blocks.get_mut(block) {
            Some(found) => {
                for (i, byte) in contents.as_bytes().iter().enumerate().take(BLOCK_SIZE) {
                    found[i] = *byte;
                }
                Ok(())
            }
            None => Err(RamDiskError::IllegalBlockWrite(block, self.num_blocks() - 1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TEXT_1: &str = "This is a test!!";
    const TEST_BLOCK_SIZE: usize = TEST_TEXT_1.len();
    const TEST_TEXT_2: &str = "This is a test?!";
    const TEST_TEXT_3: &str = "This is far too long to use.";

    #[test]
    fn it_works() {
        let mut disk = RamDisk::<TEST_BLOCK_SIZE, 4>::new();
        assert_eq!(disk.num_blocks(), 4);
        assert_eq!(disk.block_size(), TEST_BLOCK_SIZE);
        assert_eq!(disk.total_storage(), 64);

        disk.write_from_str(0, TEST_TEXT_1).unwrap();
        let mut read0 = [0; TEST_BLOCK_SIZE];
        disk.read(0, &mut read0).unwrap();
        assert_eq!(TEST_TEXT_1, std::str::from_utf8(&read0).unwrap());

        read0[read0.len() - 2] = '?' as u8;
        disk.write(1, &read0).unwrap();
        let mut read1 = [0; TEST_BLOCK_SIZE];
        disk.read(1, &mut read1).unwrap();
        assert_eq!(TEST_TEXT_2, std::str::from_utf8(&read1).unwrap());

        let err = disk.read(disk.num_blocks(), &mut read0);
        assert!(err.is_err());

        disk.write_from_str(0, TEST_TEXT_3).unwrap();
        disk.read(0, &mut read0).unwrap();
        assert_eq!(
            &TEST_TEXT_3[..TEST_BLOCK_SIZE],
            std::str::from_utf8(&read0).unwrap()
        );
    }
}
