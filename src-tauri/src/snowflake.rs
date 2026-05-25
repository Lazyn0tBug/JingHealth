use std::sync::atomic::{AtomicU64, Ordering};

pub struct SnowflakeGenerator {
    node_id: u64,
    epoch: u64,
    sequence: AtomicU64,
}

impl SnowflakeGenerator {
    pub fn new(node_id: u64) -> Self {
        Self {
            node_id: node_id.min(1023),
            epoch: 1_735_689_600_000,
            sequence: AtomicU64::new(0),
        }
    }

    pub fn generate(&self) -> u64 {
        let timestamp = current_timestamp_millis().saturating_sub(self.epoch);
        let seq = self.sequence.fetch_add(1, Ordering::Relaxed) & 0xFFF;
        (timestamp << 22) | (self.node_id << 12) | seq
    }

    pub fn encode_base62(&self, n: u64) -> String {
        const CHARS: &[u8; 62] =
            b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut buf = [b'0'; 11];
        let mut n = n;
        for i in (0..11).rev() {
            buf[i] = CHARS[(n % 62) as usize];
            n /= 62;
        }
        // SAFETY: buf is ASCII
        unsafe { String::from_utf8_unchecked(buf.to_vec()) }
    }

    pub fn generate_str(&self) -> String {
        self.encode_base62(self.generate())
    }
}

fn current_timestamp_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_uniqueness() {
        let gen = SnowflakeGenerator::new(1);
        let mut ids = std::collections::HashSet::new();
        for _ in 0..1000 {
            let id = gen.generate();
            assert!(ids.insert(id), "duplicate id: {}", id);
        }
    }

    #[test]
    fn test_id_length() {
        let gen = SnowflakeGenerator::new(1);
        for _ in 0..100 {
            let s = gen.generate_str();
            assert_eq!(s.len(), 11, "expected 11 chars, got {}: {}", s.len(), s);
        }
    }

    #[test]
    fn test_id_monotonic() {
        let gen = SnowflakeGenerator::new(1);
        let mut prev = 0u64;
        for _ in 0..1000 {
            let id = gen.generate();
            assert!(id > prev, "not monotonic: {} -> {}", prev, id);
            prev = id;
        }
    }
}