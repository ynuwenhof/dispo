use bitvec::array::BitArray;
use bitvec::view::BitViewSized;
use fnv::FnvHasher;
use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

pub(crate) struct Bloom<A: BitViewSized> {
    buf: BitArray<A>,
    bits: usize,
    hashes: usize,
}

impl<A: BitViewSized> Bloom<A> {
    pub(crate) const fn new(buf: BitArray<A>, bits: usize, hashes: usize) -> Self {
        Self { buf, bits, hashes }
    }

    pub(crate) fn contains<T: Hash>(&self, value: &T) -> bool {
        let mut fnv = FnvHasher::default();
        value.hash(&mut fnv);
        let fnv_hash = fnv.finish() as usize;

        let mut xxhash = XxHash64::default();
        value.hash(&mut xxhash);
        let xxhash_hash = xxhash.finish() as usize;

        for i in 0..self.hashes {
            let index = fnv_hash.wrapping_mul(xxhash_hash.wrapping_add(i)) % self.bits;

            if !self.buf[index] {
                return false;
            }
        }

        true
    }
}
