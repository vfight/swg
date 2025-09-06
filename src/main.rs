use solana_sdk::bs58;
use solana_sdk::signature::{Keypair, Signer};
use std::env;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use tokio::task;

lazy_static::lazy_static! {
    static ref COUNT: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let (starts, ends, threads) = if args.len() == 1 {
        ("".to_string(), "".to_string(), 1)
    } else if args.len() == 4 {
        (args[1].clone(), args[2].clone(), args[3].parse().unwrap())
    } else {
        panic!("Usage: {:?} [starts ends threads]", args);
    };

    println!(
        "Searching for addresses starting with: {} ending with: {} threads: {}",
        starts, ends, threads
    );
    println!("Press Ctrl+C to stop.");
    println!("Starting...");

    let mut handles = Vec::new();

    for _ in 0..threads {
        let starts = starts.clone();
        let ends = ends.clone();
        let handle = task::spawn(async move {
            loop {
                let keypair = Keypair::new();
                let pubkey_str = keypair.pubkey().to_string();
                if (pubkey_str.starts_with(&starts)) && (pubkey_str.ends_with(&ends)) {
                    println!(
                        "{} {}",
                        pubkey_str,
                        bs58::encode(keypair.to_bytes()).into_string()
                    );
                    let count = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if count == 1000 {
                        exit(0)
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}
