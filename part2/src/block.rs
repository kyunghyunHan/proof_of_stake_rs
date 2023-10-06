use chrono::Utc;
use rand::Rng;
use sha2::{Digest, Sha256};
use crate::wallet;
pub struct Block {
    pub time_stamp:String,
    pub prev_hash:String,
    pub hash :String,
    pub validator_addr:String
}
pub struct Node {
   pub  stake:i32,
   pub address:String


}



impl Block {
    fn print_block_info(&self) {
        println!("\tTimestamp: {}", self.time_stamp);
        println!("\tPrevious Hash: {}", self.prev_hash);
        println!("\tHash: {}", self.hash);
        println!("\tValidator Address: {}", self.validator_addr);
    }
}
pub struct PosNetWork {
     pub blockchain: Vec<Block>,
     pub blockchain_head :Block,
     pub validators : Vec<Node>,
}
impl PosNetWork {
  pub fn print_blockchain_info(&self){
      for (i, block) in self.blockchain.iter().enumerate(){
        println!("Block {} Info:", i);
        block.print_block_info();

      }
   }
   
   pub fn new_node(&mut self,stake:i32){
    let new_node = Node {
        stake,
        address:wallet::rand_address(),
    };
    self.validators.push(new_node);

   }
   
}
