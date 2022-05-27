use tonic::transport::Server;

mod my_grpc_server;
use my_grpc_server::MyGrpcServer;

mod service {
    tonic::include_proto!("my_grpc_service");
}
use service::my_grpc_service_server::MyGrpcServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse().unwrap(); // localhost
    let server = MyGrpcServer::default();
    // Start our server
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(MyGrpcServiceServer::new(server))
        .serve(addr)
        .await?;
    Ok(())
}
