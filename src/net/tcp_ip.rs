use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::clocks::RoscRng;
use rand::RngCore;
use static_cell::StaticCell;

const MAX_SOCKETS: usize = 32;

#[embassy_executor::task]
async fn tcp_ip_driver(stack: &'static Stack<cyw43::NetDriver<'static>>) {
    info!("TCP/IP: Driver running");
    stack.run().await;
}

pub async fn start_tcp_ip(
    spawner: &Spawner,
    device: cyw43::NetDriver<'static>,
) -> &'static Stack<cyw43::NetDriver<'static>> {
    info!("TCP/IP: Starting driver");

    let config = Config::dhcpv4(Default::default());

    static RESOURCES: StaticCell<StackResources<MAX_SOCKETS>> = StaticCell::new();
    let resources = RESOURCES.init(StackResources::new());

    let random_seed = RoscRng.next_u64();

    static STACK: StaticCell<Stack<cyw43::NetDriver<'static>>> = StaticCell::new();
    let stack = &*STACK.init(Stack::new(device, config, resources, random_seed));

    unwrap!(spawner.spawn(tcp_ip_driver(stack)));

    stack
}
