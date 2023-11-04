use std::hash::Hasher;
use lz_fnv::Fnv1a;

pub fn calculate_id_hash(bytes: &[u8]) -> [u8; 8] {
    let mut fnv1a = Fnv1a::<u64>::new();
    fnv1a.write(bytes);
    fnv1a.finish().to_be_bytes()
}
