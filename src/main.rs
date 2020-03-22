use pum::{MallocSizeOfExt as _};
use parity_util_mem::{MallocSizeOf as _};
use keccak_hasher::KeccakHasher;
use memory_db::{MemoryDB, HashKey};

fn main() {
    let m = MemoryDB::<KeccakHasher, HashKey<_>, Vec<u8>>::default();
    println!("{}", m.malloc_size_of());
}
