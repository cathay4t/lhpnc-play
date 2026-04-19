// SPDX-License-Identifier: Apache-2.0

use hello_world::{HelloRequest, greeter_client::GreeterClient};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let outbound = async_stream::stream! {
        let mut interval =
            tokio::time::interval(std::time::Duration::from_secs(1));
        let mut count = 0;

        loop {
            interval.tick().await;
            let request = HelloRequest {
                name: format!("client_{count}"),
            };
            count +=1;
            yield request;
        }
    };

    let response = client.say_hello(tonic::Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(reply) = inbound.message().await? {
        println!("RESPONSE={:?}", reply);
    }

    Ok(())
}
