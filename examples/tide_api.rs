use mobc::Pool;
use mobc_redis::RedisConnectionManager;
use mobc_redis::{redis, Connection};
use tide::Request;
use tide::StatusCode;

#[derive(Clone)]
// implémentation automatique du trait Clone
// qui est nécessaire pour que tide puisse
// donner une copie de l'état de l'application
// a chaque requête
struct AppState {
    pool: Pool<RedisConnectionManager>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let client =
        redis::Client::open("redis://127.0.0.1/").unwrap();
    let manager = RedisConnectionManager::new(client);
    // On crée ici un connection pool vers Redis
    let pool = Pool::builder()
        // On utilisera 5 connections concurrentes à Redis
        .max_open(5)
        // on évitera ici de produire un Timeout
        // a cause d'une attente sur Redis
        .get_timeout(None)
        .build(manager);

    // Ceci est l'état global de l'api,
    // on l'utilise pour stoker le connection pool
    // il sera cloné a chaque appel http
    let app_state = AppState { pool };

    // tide (le framework web utilisé ici)
    // est créé avec ce state pour état global,
    // il a une route vers le handler `uniques`,
    // et écoutera sur le port 8080 de localhost
    let mut app = tide::with_state(app_state);
    app.at("/uniques/").get(uniques);
    app.at("/ping/").get(ping);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn uniques(
    req: Request<AppState>,
) -> tide::Result<String> {
    // ici on récupère un identificateur naïf de l'utilisateur
    let client_ip = req.peer_addr().unwrap_or_default();

    // La récupération d'une connection du pool peut générer
    // une erreur que nous allons gérer manuellement ici
    match req.state().pool.get().await {
        // si c'est une erreur
        Err(e) => {
            // on log le message
            println!(
                "Could not get redis connection because : {}",
                e
            );

            // puis on retourne HTTP_500
            Err(tide::Error::from_str(
                StatusCode::InternalServerError,
                e,
            ))
        }
        // si on a bien récupéré la connection vers Redis
        Ok(mut con) => {
            // on l'utilise
            #[rustfmt::skip]
            let uniques : (u64,) = redis::pipe()
                    .cmd("pfadd").arg("visitors").arg(client_ip)
                    .ignore()
                    .cmd("pfcount").arg("visitors")
                .query_async(&mut con as &mut Connection)
                .await?;
            // ci dessus deux opérateurs se succèdent
            // - .await : pour attendre et récupérer le résultat
            //   de la Future retournée par `query_async`
            // - ?      : pour ensuite récupérer
            //   la valeur dans Ok(_)
            //   ou retourner directement Err(_)

            // puis on retourne HTTP_200
            // et un simple texte comme résultat
            Ok(format!(
                "There have been {} unique visitors",
                uniques.0 // la première valeur du tuple
            ))
        }
    }
}

// cette route va nous permettre de comparer les performances
// de cette fonction comparée à celle qui inclut l'appel
// a Redis
#[rustfmt::skip]
async fn ping(_: Request<AppState>) -> tide::Result<&'static str> 
{ Ok("ok") }
