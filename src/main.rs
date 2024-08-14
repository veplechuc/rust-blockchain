mod block;
mod blockchain;

use crate::blockchain::BlockChain;
use crate::block::Block;


fn main () { 
    let mut new_BC = BlockChain::new(); 
    new_BC.starting_block();

    println!("{:?}", new_BC); 
   
    let new_block = Block::new(2, new_BC.blocks[0].hash.to_owned(), "Azam".to_string());
    new_BC.try_add_block(new_block);

    new_BC.is_chain_valid(&new_BC.blocks);


    let new_block = Block::new(3,new_BC.blocks[1].hash.to_owned(), "kamran".to_string()); 
    new_BC.try_add_block(new_block); 


    let new_block = Block::new(4,new_BC.blocks[2].hash.to_owned(), "Khan".to_string()); 
    new_BC.try_add_block(new_block); 

    new_BC.is_chain_valid(&new_BC.blocks);  

    new_BC.blocks = new_BC.chain_selector(new_BC.blocks.to_owned(), new_BC.blocks.to_owned()).unwrap();


}
