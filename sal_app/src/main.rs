use actix_web::dev::Server;
use anyhow::Result;

#[cfg(feature = "setup_network")]
use lib::{add_address, add_route_v4, get_links, rtnetlink::new_connection};

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "setup_network")]
    setup_network("192.168.0.55", "255.255.255.0", "192.168.0.1").await?;

    match start_web().await {
        Err(e) => eprintln!("error starting web server {e}"),
        Ok(web) => web.await?,
    }

    Ok(())
}

async fn start_web() -> Result<Server> {
    let address = "0.0.0.0:80";
    let listener = std::net::TcpListener::bind(address).expect("Failed to bind");
    match libsal::run(listener) {
        Err(e) => anyhow::bail!("error {e}"),
        Ok(l) => Ok(l),
    }
}

#[cfg(feature = "setup_network")]
async fn setup_network(ip_addr: &str, mask: &str, gw: &str) -> Result<()> {
    let (con, handle, _) = new_connection().expect("no new connection");
    tokio::spawn(con);

    let links = get_links(&handle).await?;

    let links = links
        .into_iter()
        .filter_map(|(_, name)| {
            if name.starts_with("e") {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let addr = if let Ok(addr) = ip_addr.parse() {
        addr
    } else {
        eprintln!("invalid ip address");
        std::process::exit(1);
    };

    let mask = if let Ok(mask) = mask.parse() {
        mask
    } else {
        eprintln!("invalid subnet mask");
        std::process::exit(1);
    };

    let dest = if let Ok(dest) = "0.0.0.0".parse() {
        dest
    } else {
        eprintln!("invalid route");
        std::process::exit(1);
    };

    let gw = if let Ok(gw) = gw.parse() {
        gw
    } else {
        eprintln!("invalid gateway");
        std::process::exit(1);
    };

    add_address(&handle, &links[0], addr, mask).await?;
    add_route_v4(&handle, dest, 0, gw).await?;

    Ok(())
}
