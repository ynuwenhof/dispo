use bitvec::bitvec;
use fnv::FnvHasher;
use std::f64::consts::LN_2;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, fs};
use twox_hash::XxHash64;

const CAPACITY: usize = 55_743 * 3;
const PROBABILITY: f64 = 0.001;

fn bits(capacity: usize, probability: f64) -> usize {
    let numerator = -(capacity as f64) * probability.ln();
    let denominator = LN_2.powi(2);

    (numerator / denominator).ceil() as usize
}

fn hashes(capacity: usize, bits: usize) -> usize {
    let hashes = (bits as f64 / capacity as f64) * LN_2;
    hashes.ceil() as usize
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dispo.rs");

    let bits = bits(CAPACITY, PROBABILITY);
    let hashes = hashes(CAPACITY, bits);
    let mut buf = bitvec![0; bits];

    let file = File::open("blacklist.txt").unwrap();
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line.unwrap();

        let mut fnv = FnvHasher::default();
        line.hash(&mut fnv);
        let fnv_hash = fnv.finish() as usize;

        let mut xxhash = XxHash64::default();
        line.hash(&mut xxhash);
        let xxhash_hash = xxhash.finish() as usize;

        for i in 0..hashes {
            let index = fnv_hash.wrapping_mul(xxhash_hash.wrapping_add(i)) % bits;

            buf.set(index, true);
        }
    }

    let buf = buf.as_raw_slice();
    let len = buf.len();

    fs::write(
        dest_path,
        format!(
            "static BLOOM: crate::bloom::Bloom<[usize; {len}]> = crate::bloom::Bloom::new(
                bitvec::array::BitArray {{
                    data: {buf:?},
                    _ord: std::marker::PhantomData,
                }},
                {bits},
                {hashes}
            );"
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=blacklist.txt");
}
