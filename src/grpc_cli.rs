// SPDX-License-Identifier: Apache-2.0
mod benchmark;
mod hello_world {
    tonic::include_proto!("helloworld");
}

use self::{
    benchmark::LhpncBenchmark,
    hello_world::{HelloRequest, greeter_client::GreeterClient},
};

const REQUST_COUNT: usize = 10 * 1000 * 1000;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bench = LhpncBenchmark::start("no_optimize");
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let outbound = async_stream::stream! {
        let mut count = 0;

        loop {
            let request = HelloRequest {
                name: format!("client_{count}"),
            };
            count +=1;
            if count > REQUST_COUNT  {
                break;
            }
            yield request;
        }
    };

    let response = client.say_hello(tonic::Request::new(outbound)).await?;
    let mut inbound = response.into_inner();
    let mut got_reply = 0;

    while inbound.message().await?.is_some() {
        if got_reply > REQUST_COUNT {
            break;
        }
        got_reply += 1;
    }
    if got_reply == REQUST_COUNT {
        bench.end(REQUST_COUNT as u128);
        println!("{}", serde_yaml::to_string(&bench)?);
    } else {
        eprintln!(
            "Has missing reply, expected {REQUST_COUNT}, got {got_reply}"
        );
    }

    Ok(())
}
