use crate::Transaction;

pub struct Block {
    pub id:usize,
    pub hash:String,
    pub previuos_hash:String,
    pub timestamp:i64,
    // pub txn:Vec<Transaction>
}
impl Block{
    pub fn new(){
      println!("hello");
        }
}

pub fn calcucate_hash ( 
  id: &usize,
  timestamp: &i64,
  previous_hash: &str,
  txn: &Vec<Transaction>,)->String{
  //  Util::hash(&hash.to_string())
  "d".to_string()
}
