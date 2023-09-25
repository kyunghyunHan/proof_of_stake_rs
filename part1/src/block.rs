use crate::Transaction;
use crate::Wallet;
use sha256::digest_bytes;
use log::info;
pub struct Block {
    pub id:usize,
    // pub hash:String,
    pub previous_hash:String,
    pub timestamp:i64,
    // pub txn:Vec<Transaction>
}
impl Block{
  pub fn new(
    id: usize,
    previous_hash: String,
    timestamp: i64,
    txn: Vec<Transaction>,
    difficulty: u32,
    mut validator_wallet: Wallet,
) -> Self {
    info!("creating block...");
    // let hash: String = calcucate_hash(&id, &timestamp, &previous_hash, &txn);
    // let signature = validator_wallet.sign(&hash);
    Self {
        id,
        // hash,
        previous_hash,
        timestamp,
        // txn,
        // validator: validator_wallet.get_public_key(),
        // signature: signature,
        // difficulty: difficulty,
    }
}


 pub fn genesis(wallet: Wallet) -> Self {
  let id = 0;
        let timestamp = 1650205976;
        let previous_hash = String::from("genesis");
        // let txn = vec![];
        let validator = String::from("genesis");
        let signature = String::from("genesis");
        let difficulty = 5;

        // let hash = calcucate_hash(
        //     &id,
        //     &timestamp,
        //     &previous_hash,
        //     &txn,
        //     &validator,
        //     &difficulty,
        // );
  Self {
    id,
    // hash,
    previous_hash,
    timestamp,
    // txn,
    // validator,
    // signature,
    // difficulty,
}}
}

pub fn calcucate_hash ( 
  id: &usize,
  timestamp: &i64,
  previous_hash: &str,
  // txn: &Vec<Transaction>,
)->String{
  //  Util::hash(&hash.to_string())
  "d".to_string()
}
pub fn hash(data:&String)->String {
  digest_bytes(data.as_bytes())
}
