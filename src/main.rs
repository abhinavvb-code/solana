use std::io::{Error, ReadLine};
use std::str::FromStr;

use metaplex::token_metadata::{
    create_mint_nft, update_nft_metadata, delete_nft,
};
use solana_client::rpc_client::RpcClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_client = RpcClient::new("https://devnet.solana.com");
    let metaplex = Metaplex::new(rpc_client);

    let mut flag = false;
    let mut input_line = String::new();

    while !flag {
        println!("Enter 1 to mint NFT, 2 to update, 3 to delete NFT, 4 to exit loop:");
        std::io::stdin().read_line(&mut input_line)?; // Use `?` for error handling

        let input = input_line.trim();

        match input.parse::<i32>() {
            Ok(x) => {
                match x {
                    1 => mint_nft(&metaplex)?,
                    2 => update_nft(&metaplex)?,
                    3 => delete_nft(&metaplex)?,
                    4 => {
                        println!("Loop exited");
                        flag = true;
                    }
                    _ => println!("Invalid option. Please enter 1, 2, 3, or 4."),
                }
            }
            Err(_) => println!("Invalid input. Please enter a number."),
        }

        input_line.clear();
    }

    Ok(())
}

async fn mint_nft(metaplex: &Metaplex) -> Result<(), Box<dyn std::error::Error>> {
    // Implement your specific NFT minting logic using metaplex.token_metadata::create_mint_nft(...)
    // ...
}

async fn update_nft(metaplex: &Metaplex) -> Result<(), Box<dyn std::error::Error>> {
    // Implement your specific NFT update logic using metaplex.token_metadata::update_nft_metadata(...)
    // ...
}

async fn delete_nft(metaplex: &Metaplex) -> Result<(), Box<dyn std::error::Error>> {
    // Implement your specific NFT deletion logic using metaplex.token_metadata::delete_nft(...)
    // ...
}
