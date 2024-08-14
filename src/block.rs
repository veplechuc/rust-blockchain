use chrono::Utc;
use sha256::digest;

#[derive(Debug, Clone)] 
pub struct Block {
 pub id: u64, 
 pub nonce: u64, 
 pub data: String, 
 pub hash: String, 
 pub previous_hash: String, 
 pub timestamp: i64, 
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now(); 
        let now_timestamp = now.timestamp(); 
        
        let (nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data); 

        Self {
            id, 
            hash, 
            timestamp: now.timestamp(), 
            previous_hash, 
            data, 
            nonce
        }
    }
    pub fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("mining block ..."); 
        let mut nonce = 1; 

        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce); 
            let hash = digest(block_string); 
            if hash.starts_with("0000") {
                println!("mined! nonce: {}, hash: {}", nonce, hash);
                return (nonce, hash)
            }
            nonce += 1;
        }
    } 
}