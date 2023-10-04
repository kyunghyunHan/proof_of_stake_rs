use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct  Wallet {

    pub key_pair: String,


    
}

impl Wallet {
  pub fn new()->Self{
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);
    let pub_key = hex::encode(keypair.public.to_bytes());
    println!("Your Public Key {}", pub_key);
   
    let keypair = hex::encode(keypair.to_bytes());
    println!("Your Key Pair {}", keypair);

   Self { key_pair: keypair }
  }
}