use crypto_hash::{hex_digest, Algorithm};
#[derive(Debug)]
pub struct Block {
  pub   timestamp: String,
  pub last_hash: String,
  pub hash: String,
  pub data: Vec<u8>,
}

impl Block {
  pub  fn new(timestamp: &str, last_hash: &str, hash: &str, data: Vec<u8>) -> Self {
        Block {
            timestamp: timestamp.to_string(),
            last_hash: last_hash.to_string(),
            hash: hash.to_string(),
            data,
        }
    }

    pub fn to_string(&self) -> String {
        let last_hash_slice = if self.last_hash.len() > 10 {
            &self.last_hash[..10]
        } else {
            &self.last_hash
        };
        let hash_slice = if self.hash.len() > 10 {
            &self.hash[..10]
        } else {
            &self.hash
        };
    
        format!(
            "Block -\n    Timestamp   : {}\n    last Hash   : {}\n    Hash        : {}\n    Data        : {:?}",
            self.timestamp,
            last_hash_slice,
            hash_slice,
            self.data
        )
    }
    
    pub fn genesis() -> Self { 
        Block::new("Genesis time", "-----", "f1f33r-efvh", Vec::new())
    }

    pub fn mine_block(last_block: &Block, data: Vec<u8>) -> Self {
        let timestamp = chrono::offset::Utc::now().timestamp_millis().to_string();
        let last_hash = &last_block.hash;
        let hash = Block::hash(&timestamp, last_hash, &data);
        Block::new(&timestamp, last_hash, &hash, data)
    }

    pub fn hash(timestamp: &str, last_hash: &str, data: &[u8]) -> String {
        let input = format!("{}{}{:?}", timestamp, last_hash, data);
        let hash_bytes = hex_digest(Algorithm::SHA256, input.as_bytes());
        format!("{}", hash_bytes)
    }

   pub  fn block_hash(block: &Block) -> String {
        Block::hash(&block.timestamp, &block.last_hash, &block.data)
    }
}
pub fn main() {
    let genesis_block = Block::genesis();
    println!("{}", genesis_block.to_string());

    let last_block = &genesis_block;
    let data = vec![1, 2, 3, 4, 5];
    let new_block = Block::mine_block(last_block, data.clone());
    println!("{}", new_block.to_string());

    let block_hash = Block::block_hash(&new_block);
    println!("Block Hash: {}", block_hash);
}