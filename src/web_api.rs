use defmt::{info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::Stack;
use low_profile::heapless::Json;
use low_profile::{Router, Service};
use serde::Serialize;

#[derive(Serialize)]
struct HelloWorldResponse {
    message: HelloWorld,
}

#[derive(Serialize)]
enum HelloWorld {
    HelloWorld,
}

#[embassy_executor::task]
async fn web_api_task(stack: &'static Stack<cyw43::NetDriver<'static>>) {
    let mut rx_buffer = [0; 1024];
    let mut tx_buffer = [0; 1024];

    let router = Router::new().get("/", || async {
        Json::<HelloWorldResponse>(HelloWorldResponse {
            message: HelloWorld::HelloWorld,
        })
    });

    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    loop {
        if let Err(e) = socket.accept(80).await {
            warn!("Web API: Accept error: {}", e);
            socket.abort();
            continue;
        }

        match socket.remote_endpoint() {
            None => info!("Web API: Connection from unknown"),
            Some(endpoint) => info!("Web API: Connection from {}", endpoint),
        }

        let (read, write) = socket.split();

        if let Err(e) = router.serve(read, write).await {
            warn!("Web API: Serve error: {:?}", e);
        }

        socket.close();

        if let Err(e) = socket.flush().await {
            warn!("Web API: Flush error: {:?}", e);
        }

        socket.abort();
    }
}

pub fn start_web_api(spawner: &Spawner, stack: &'static Stack<cyw43::NetDriver<'static>>) {
    unwrap!(spawner.spawn(web_api_task(stack)))
}
