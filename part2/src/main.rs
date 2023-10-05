use part2::Block;

mod block;
fn main() {
    let genesis_block = Block::genesis();
    println!("{:?}", Block::to_string(&genesis_block));

    let last_block = &genesis_block;
    let data = vec![1, 2, 3, 4, 5];
    let new_block = Block::mine_block(last_block, data.clone());
    println!("{:?}", new_block);

    let block_hash = Block::block_hash(&new_block);
    println!("Block Hash: {}", block_hash);
    block::main();
}
