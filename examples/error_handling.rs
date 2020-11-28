use rand::random;
use std::time::Duration;

fn main() -> Result<(), MyErrors> {
  let s = String::from("Ok!");
  // je vais beaucoup utiliser .clone() pour
  // dupliquer la valeur sans quoi il ne me
  // serait pas possible de la réutiliser
  // aussi naïvement
  //
  // gestion d'erreur recommandée
  let s1 = timeout(s.clone())?;

  // la même chose, mais sans utiliser `?`
  let s2 = match timeout(s.clone()) {
    Ok(s) => s,
    Err(e) => return Err(e),
  };

  // unwrap() retour la valeur de Ok()
  // mais si c'est une erreur (Err(_)),
  // le thread va paniquer et interrompre
  // son exécution. Selon votre use case,
  // une gestion d'erreur plus `brutale`
  // comme celle-ci peut être envisageable
  let s3 = timeout(s.clone()).unwrap();

  // maintenant imaginez ce que serait
  // la gestion d'erreur ci dessous sans
  // l'usage salvateur de `?`
  let s = timeout(timeout(timeout(s.clone())?)?)?;
  println!("Ouff : {}", s);
  Ok(())
}

#[rustfmt::skip]
fn timeout(s: String) -> Result<String, MyErrors> {
    if random() { Ok(s) } 
    else { Err(MyErrors::PasALHeure(Duration::from_secs(1))) }
}

#[derive(Debug)]
enum MyErrors {
  PasContent(String),
  PasALHeure(Duration),
  ParceQUE,
}

#[rustfmt::skip]
fn content(s: String) -> Result<String, MyErrors> {
    if random() { Ok(s) } 
    else { Err(MyErrors::PasContent(String::from(
        "Non, vraiment pas content",
    ))) }
}

#[rustfmt::skip]
fn pourquoi(s: String) -> Result<String, MyErrors> {
    if random() { Ok(s) } 
    else { Err(MyErrors::ParceQUE) }
}
