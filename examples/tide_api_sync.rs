use futures::prelude::*;
use redis::AsyncCommands;
use tide::Request;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/uniques/").get(uniques);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

//https://github.com/redis/redis/issues/135
//https://stackoverflow.com/a/37184581/1797308

async fn uniques(req: Request<()>) -> tide::Result<String> {
    let client_ip = req.peer_addr().unwrap_or_default();
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    #[rustfmt::skip]
    let uniques : (u64,) = redis::pipe()
        // .atomic()
            .cmd("pfadd").arg("visitors").arg(client_ip)
            .ignore()
            .cmd("pfcount").arg("visitors")
        .query_async(&mut con)
        .await?;

    Ok(format!("There have been {} unique visitors", uniques.0))
}
