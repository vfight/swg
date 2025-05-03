use solana_sdk::bs58;
use solana_sdk::signature::{Keypair, Signer};
use std::env;
use tokio::task;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ends = args[1].clone();
    let threads: usize = args[2].parse().unwrap();

    println!(
        "Searching for addresses ending with: {} threads: {}",
        ends, threads
    );
    println!("Press Ctrl+C to stop.");
    println!("Starting...");

    let mut handles = Vec::new();

    for _ in 0..threads {
        let ends = ends.clone();
        let handle = task::spawn(async move {
            loop {
                let keypair = Keypair::new();
                let pubkey_str = keypair.pubkey().to_string();
                if pubkey_str.ends_with(&ends) {
                    println!(
                        "🎉 Found address: {} {}",
                        pubkey_str,
                        bs58::encode(keypair.to_bytes()).into_string()
                    );
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}
