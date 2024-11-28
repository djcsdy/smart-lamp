use cyw43::NetDriver;
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_net::{Config, Runner, Stack, StackResources};
use embassy_rp::clocks::RoscRng;
use rand::RngCore;
use static_cell::StaticCell;

const MAX_SOCKETS: usize = 32;

#[embassy_executor::task]
async fn tcp_ip_driver(mut runner: Runner<'static, NetDriver<'static>>) {
    info!("TCP/IP: Driver running");
    runner.run().await;
}

pub async fn start_tcp_ip(
    spawner: &Spawner,
    device: cyw43::NetDriver<'static>,
) -> Stack<'static> {
    info!("TCP/IP: Starting driver");

    let config = Config::dhcpv4(Default::default());

    static RESOURCES: StaticCell<StackResources<MAX_SOCKETS>> = StaticCell::new();
    let resources = RESOURCES.init(StackResources::new());

    let random_seed = RoscRng.next_u64();

    let (stack, runner) = embassy_net::new(device, config, resources, random_seed);

    unwrap!(spawner.spawn(tcp_ip_driver(runner)));

    stack
}
