use crate::block::Block;
use chrono::Utc;
use sha256::digest;


#[derive(Debug, Clone)] 
pub struct BlockChain {
 pub blocks: Vec<Block>,
}
impl BlockChain {
   pub fn new() -> Self {
        Self {
            blocks: vec![],
        }
    }

   pub fn starting_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            data: String::from("I am a first or genesis block"), 
            previous_hash: String::from("0000000000000000000000000000000000000000000000000000000000000000"), 
            nonce: 11316, 
            hash: String::from("000015783b764259d382017d91a36d206d0600e2cbb3567748f46a33fe9297cf"),
            timestamp:  Utc::now().timestamp(),
        }; 
        self.blocks.push(genesis_block);
    }

    pub fn try_add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => {
                println!("the blockchain does not have atleast one block"); 
                return; 
            }, 

            Some(latest_block) => {
                if self.is_block_valid(&block, latest_block) {
                    self.blocks.push(block); 
                    println!("Block has been successfully added"); 

                } else  {
                    println!("Could not add block, invalid!");
                }

            }, 
        }
    }

    pub fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("Bock with id {} has wrong previous hash", new_block.id); 
            return false;
        } else if !new_block.hash.starts_with("0000") {
            println!("block with id: {} has invalid hash", new_block.id);  
            return false;    
        } else if new_block.id != latest_block.id +1 {
            println!("block with id {} is not the next block after the latest block with id: {}", 
            new_block.id, latest_block.id);
            return false;
        } else if digest(format!("{}{}{}{}{}", new_block.id, &new_block.previous_hash, &new_block.data, new_block.timestamp, new_block.nonce)) 
        != new_block.hash {
            println!("block with id {} has invalid hash", new_block.id); 
            return false; 
        }
        true
   }
   
   pub fn is_chain_valid(&self, chain: &Vec<Block>) -> bool { 
    match chain.len() {
        0 => println!("The chain is empty"), 
        1 => println!("The chain only contains a single block"), 
        _ => {
            for i in 1..chain.len() {
                let previous = chain.get(i - 1).unwrap();
                let current = chain.get(i).unwrap();
                if !self.is_block_valid(current, previous) {
                    return false;
                }
            }

        }
    }
    
    println!("The chain is found to be correct and valid"); 
    true
}
 
pub fn chain_selector(&self, local: Vec<Block>, remote: Vec<Block>) -> Option<Vec<Block>> {
    let is_local_valid = self.is_chain_valid(&local);
    let is_remote_valid = self.is_chain_valid(&remote);

    match (is_local_valid, is_remote_valid) {
        (true, true) => {
            if local.len() >= remote.len() {
                println!("The local copy is valid"); 
                Some(local)
            } else {
                println!("The remote copy is valid");
                Some(remote)
            }    
        },

        (true, false) => { 
            println!("The remote copy is valid, returning local copy"); 
            Some(local)
        }, 
        
        (false, true) => {
            println!("The local copy is valid, returning remote copy"); 
            Some(remote)    
        },

        (false, false) => {
            println!("Both local and remote copies are not valid");
            None
        },
    }

}
    }
