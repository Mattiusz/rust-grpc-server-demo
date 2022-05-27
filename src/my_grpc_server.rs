use crate::service::{my_grpc_service_server::MyGrpcService, MyGrpcRequest, MyGrpcResponse};
use futures::Stream;
use rand::prelude::*;
use std::{pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

#[derive(Default, Clone)]
pub struct MyGrpcServer {}

async fn get_some_response() -> MyGrpcResponse {
    let mut rand_number = rand::thread_rng();
    MyGrpcResponse {
        some_double: rand_number.gen(),
        some_string: rand_number.gen::<f64>().to_string(),
    }
}

async fn get_vector_of_some_responses() -> Vec<MyGrpcResponse> {
    let some_response = get_some_response().await;
    let len = 5;
    let mut responses = Vec::with_capacity(len);
    for _ in 0..len {
        responses.push(some_response.clone());
    }
    responses
}

#[tonic::async_trait]
impl MyGrpcService for MyGrpcServer {
    type GetLotsOfDataStream = Pin<Box<dyn Stream<Item = Result<MyGrpcResponse, Status>> + Send>>;

    async fn send_and_get_data(
        &self,
        request: Request<MyGrpcRequest>,
    ) -> Result<Response<MyGrpcResponse>, Status> {
        let _my_grpc_request = request.into_inner(); // Incoming request message
        let my_grpc_response = get_some_response().await; // Outgoing response message

        Ok(Response::new(my_grpc_response))
    }

    async fn get_lots_of_data(
        &self,
        request: Request<MyGrpcRequest>,
    ) -> Result<Response<Self::GetLotsOfDataStream>, Status> {
        let _my_grpc_request = request.into_inner(); // Incoming request message
        let my_grpc_responses = get_vector_of_some_responses().await; // Outgoing response messages

        // Throttle data so that a response is send every 500ms
        let mut outgoing_stream =
            Box::pin(tokio_stream::iter(my_grpc_responses).throttle(Duration::from_millis(500)));

        // Spawn channels and send response messages
        let max_number_of_messages_in_buffer = 128;
        let (tx, rx) = mpsc::channel(max_number_of_messages_in_buffer);
        tokio::spawn(async move {
            while let Some(item) = outgoing_stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // Message was queued
                    }
                    Err(_item) => {
                        // Output stream was build and will be dropped
                        break;
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn send_lots_of_data(
        &self,
        request: Request<Streaming<MyGrpcRequest>>,
    ) -> Result<tonic::Response<self::MyGrpcResponse>, tonic::Status> {
        let mut incoming_stream = request.into_inner(); // Incoming request messages

        while let Some(request) = incoming_stream.next().await {
            // Perform some operation on current request element here
            let _current_request = Some(request?);
        }

        let response = get_some_response().await; // Outgoing response message

        Ok(Response::new(response))
    }
}
