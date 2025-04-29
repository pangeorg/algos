use std::hash::{BuildHasher, Hash, Hasher, RandomState};

pub struct HyperLogLog<S>
where
    S: BuildHasher,
{
    precision: u8,
    register_count: usize,
    alpha: f64,
    registers: Vec<u32>,
    builder: S,
}

impl HyperLogLog<RandomState> {
    pub fn new(precision: u8) -> Self {
        let register_count = 1 << precision;
        HyperLogLog {
            precision,
            register_count,
            alpha: alpha(precision),
            registers: Vec::with_capacity(register_count as usize),
            builder: RandomState::new(),
        }
    }
}

impl<S> HyperLogLog<S>
where
    S: BuildHasher,
{
    pub fn add<T>(&mut self, value: &T)
    where
        T: Hash + ?Sized,
    {
        let mut hasher = self.builder.build_hasher();

        value.hash(&mut hasher);

        let mut hash = hasher.finish() as u32;

        let index = (hash as usize) & (self.register_count - 1);
        hash = (hash << self.precision) | (1 << (self.precision - 1));
        let nzeros: u32 = 1 + hash.leading_zeros();
        self.registers[index] = std::cmp::max(self.registers[index], nzeros);
    }

    pub fn with_hasher(hash_builder: S, precision: u8) -> Self {
        let m = 1 << precision;
        HyperLogLog {
            precision,
            register_count: m,
            alpha: alpha(precision),
            registers: Vec::with_capacity(m as usize),
            builder: hash_builder,
        }
    }

    pub fn count(&self) -> f64 {
        let (mut raw, mut zeros) = (0.0, 0);

        for v in self.registers.iter() {
            raw += 1.0 / (1u64 << v) as f64;
            zeros += if *v == 0 { 1 } else { 0 };
        }

        raw = self.alpha * (self.register_count * self.register_count) as f64 / raw;

        let two32 = (1u64 << 32) as f64;

        if raw <= 2.5 * self.register_count as f64 && zeros != 0 {
            raw = self.register_count as f64 * (self.register_count as f64 / zeros as f64);
        } else if raw > two32 / 30. {
            raw = -1. * two32 * (1. - raw / two32).ln();
        }
        raw
    }
}

fn alpha(precision: u8) -> f64 {
    let mut p = precision;

    if p < 4 {
        p = 4;
    }

    if p > 16 {
        p = 16;
    }

    match p {
        4 => 0.673,
        5 => 0.697,
        6 => 0.709,
        _ => 0.7213 / (1. + 1.079 / (1 << p) as f64),
    }
}

fn main() {
    println!("Hello, world!");
}
