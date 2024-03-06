use blake3;

pub fn ironhash(work: &[u8]) -> [u8; 32] {
    assert_eq!(work.len(), 180);
    let out = blake3::hash(work);
    *out.as_bytes()
}