use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentResponse, BtcPaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}

// Define the Bitcoin service
#[derive(Debug, Default)]
pub struct BitcoinService {}

// Implement the Bitcoin service
#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        // Extract the payment request from the request object
        let req = request.into_inner();

        // Create a payment response object
        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };

        // Return the payment response object
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the address to listen on
    let addr = "[::1]:50051".parse()?;
    // Create a new instance of the Bitcoin service
    let btc_service = BitcoinService::default();

    // Start the gRPC server and listen for incoming requests
    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}