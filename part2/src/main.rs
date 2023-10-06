use part2::Block;
use part2::Wallet;
use part2::PosNetWork;
use chrono::prelude::*;

fn main() {
    let mut pos = PosNetWork {
        blockchain: vec![Block {
            time_stamp: format!("{:?}", Utc::now()),
            prev_hash: "".to_string(),
            hash: "".to_string(),
            validator_addr: "".to_string(),
        }],
        blockchain_head: Block {
            time_stamp: format!("{:?}", Utc::now()),
            prev_hash: "".to_string(),
            hash: "".to_string(),
            validator_addr: "".to_string(),
        },
        validators: Vec::new(),
    };
    PosNetWork::new_node(&mut pos,60);
   for i in 0..4{
    println!("Round {}",i);
   }
}