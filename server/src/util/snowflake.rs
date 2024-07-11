use chrono::Timelike;

pub struct SnowflakeIssuer {
    issuer_id: u8,
    worker_id: u8,
    sequence: u8,
}

pub const IRIS_EPOCH: u64 = 1577836800;
pub const SEQUENCE_BITS: u8 = 8;
pub const WORKER_BITS: u8 = 5;
pub const ISSUER_BITS: u8 = 5;

impl SnowflakeIssuer {
    pub fn new(issuer_id: u8, worker_id: u8) -> Self {
        Self {
            issuer_id,
            worker_id,
            sequence: 0,
        }
    }

    pub fn generate(&mut self) -> Snowflake {
        let timestamp: u64 = chrono::Utc::now().timestamp() as u64 - IRIS_EPOCH;

        let mut id = 0u64;
        id |= timestamp << (WORKER_BITS + SEQUENCE_BITS + ISSUER_BITS);
        id |= (self.issuer_id as u64) << (WORKER_BITS + SEQUENCE_BITS);
        id |= (self.worker_id as u64) << SEQUENCE_BITS;
        id |= self.sequence as u64;

        self.sequence = self.sequence.wrapping_add(1);
        Snowflake(id)
    }
}

pub struct Snowflake(u64);

impl Snowflake {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn timestamp(&self) -> u64 {
        self.0 >> (WORKER_BITS + SEQUENCE_BITS + ISSUER_BITS)
    }

    pub fn issuer_id(&self) -> u8 {
        (self.0 >> (WORKER_BITS + SEQUENCE_BITS)) as u8
    }

    pub fn worker_id(&self) -> u8 {
        (self.0 >> SEQUENCE_BITS) as u8
    }

    pub fn sequence(&self) -> u8 {
        self.0 as u8
    }
}