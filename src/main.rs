#[macro_use]
extern crate lazy_static;

use bazuka::blockchain::LevelDbChain;
use bazuka::crypto::{Fr, MiMC};
use bazuka::node::{Node, NodeError};
use ff::Field;
use std::path::Path;

lazy_static! {
    static ref NODE: Node<LevelDbChain> = Node::new(LevelDbChain::new(
        &home::home_dir().unwrap().join(Path::new(".bazuka"))
    ));
}

#[tokio::main]
async fn main() -> Result<(), NodeError> {
    let hasher = MiMC::new(b"mimc");
    println!(
        "MiMC output: {:?}",
        hasher.hash(&vec![Fr::zero(), Fr::one()])
    );

    NODE.run().await?;
    Ok(())
}
