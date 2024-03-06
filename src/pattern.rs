use crate::utils;

#[derive(Debug)]
pub struct Pattern<const HEADER_SIZE: usize, const HASH_SIZE: usize> {
    work: [u8; HEADER_SIZE],
    hash: [u8; HASH_SIZE],
    diff: u8,
}

impl<const HEADER_SIZE: usize, const HASH_SIZE: usize> Pattern<HEADER_SIZE, HASH_SIZE> {
    pub fn new(work: [u8; HEADER_SIZE], hash: [u8; HASH_SIZE]) -> Self {
        Pattern {
            work: work,
            hash: hash,
            diff: utils::check_diff(&hash),
        }
    }

    pub fn work(&self) -> &[u8; HEADER_SIZE] {
        &self.work
    }
    pub fn hash(&self) -> &[u8; HASH_SIZE] {
        &self.hash
    }
    pub fn diff(&self) -> u8 {
        self.diff
    }
}
