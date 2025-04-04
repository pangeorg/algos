use std::{
    f32::consts::LN_2,
    hash::{DefaultHasher, Hash, Hasher},
};

struct BloomFilter {
    epsilon: f32,
    bits: Vec<usize>,
    hash_count: usize,
}

impl BloomFilter {
    pub fn new(n: usize, epsilon: f32) -> Self {
        let size = size(n, epsilon);
        let hc = hash_count(size, n);
        let bits = vec![0; size];

        BloomFilter {
            epsilon,
            bits,
            hash_count: hc,
        }
    }

    fn get_hash<T: Hash>(&self, item: &T) -> usize {
        let mut s = DefaultHasher::new();
        item.hash(&mut s);
        (s.finish() as usize) % self.bits.len()
    }

    pub fn add<T: Hash>(&mut self, item: &T) {
        for _ in 0..self.hash_count {
            let hash = self.get_hash(item);
            self.bits[hash] = 1;
        }
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for _ in 0..self.hash_count {
            let hash = self.get_hash(item);

            if self.bits[hash] == 0 {
                return false;
            }
        }
        true
    }

    pub fn size(&self) -> usize {
        self.bits.len()
    }

    pub fn epsilon(&self) -> f32 {
        self.epsilon
    }

    pub fn hash_count(&self) -> usize {
        self.hash_count
    }
}

fn size(n: usize, p: f32) -> usize {
    let log_2: f32 = LN_2;
    let log_2_2 = log_2 * log_2;
    (-(n as f32) * f32::ln(p) / (log_2_2)).ceil() as usize
}

fn hash_count(m: usize, n: usize) -> usize {
    let n_f32 = n as f32;
    let m_f32 = m as f32;
    ((m_f32 / n_f32) * f32::ln(2.0)).ceil() as usize
}

fn main() {
    let mut filter = BloomFilter::new(20, 0.05);

    let s = filter.size();
    let e = filter.epsilon();
    let h = filter.hash_count();
    println!("Size: {s}, eps: {e}, hc: {h}");

    let word_present = vec![
        "abound".to_string(),
        "abounds".to_string(),
        "abundance".to_string(),
        "abundant".to_string(),
        "accessible".to_string(),
        "bloom".to_string(),
        "blossom".to_string(),
        "bolster".to_string(),
        "bonny".to_string(),
        "bonus".to_string(),
        "bonuses".to_string(),
        "coherent".to_string(),
        "cohesive".to_string(),
        "colorful".to_string(),
        "comely".to_string(),
        "comfort".to_string(),
        "gems".to_string(),
        "generosity".to_string(),
        "generous".to_string(),
        "generously".to_string(),
        "genial".to_string(),
    ];

    let word_absent = vec![
        "bluff".to_string(),
        "cheater".to_string(),
        "hate".to_string(),
        "war".to_string(),
        "humanity".to_string(),
        "racism".to_string(),
        "hurt".to_string(),
        "nuke".to_string(),
        "gloomy".to_string(),
        "facebook".to_string(),
        "geeksforgeeks".to_string(),
        "twitter".to_string(),
    ];

    let test_words = [&word_present[0..10], &word_absent].concat();

    for item in word_present {
        filter.add(&item);
    }

    let mut false_positives = 0;
    let tests = test_words.len();

    for item in test_words {
        if filter.contains(&item) {
            if word_absent.contains(&item) {
                false_positives += 1;
            }
        }
    }

    println!("False positives: {false_positives}");
    let r = false_positives as f32 / tests as f32;
    println!("Ratio: {r}");
}
