use std::io;
use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the Bitcoin server
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    // Prompt the user to enter their Bitcoin address
    println!("Enter your Bitcoin address:");
    let mut from_addr = String::new();
    io::stdin().read_line(&mut from_addr)?;

    // Prompt the user to enter the recipient's Bitcoin address
    println!("Enter the recipient's Bitcoin address:");
    let mut to_addr = String::new();
    io::stdin().read_line(&mut to_addr)?;

    // Prompt the user to enter the amount to send
    println!("Enter the amount to send:");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount)?;
    let amount: u64 = amount.trim().parse()?;

    // Create a payment request object with the user input
    let request = tonic::Request::new(
        BtcPaymentRequest {
            from_addr: from_addr.trim().to_owned(),
            to_addr: to_addr.trim().to_owned(),
            amount: amount.try_into().unwrap(),
        }
    );

    // Send the payment request to the server and wait for the response
    let response = client.send_payment(request).await?;

    // Print the server's response to the console
    println!("RESPONSE={:?}", response);

    Ok(())
}