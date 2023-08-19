use std::hash::Hasher;
use lz_fnv::Fnv1a;

pub fn calculate_id_hash(bytes: &[u8]) -> String {
    let mut fnv1a = Fnv1a::<u64>::new();
    fnv1a.write(bytes);
    format!("{:016x}", fnv1a.finish())
}
