use rand::random;
use std::time::Duration;

fn main() -> Result<(), MyErrors> {
    let s = String::from("Ok!");
    // la version recommandé
    let s = timeout(s)?;
    // la même chose, mais sans utiliser ?
    let s = match timeout(s) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let s = timeout(content(pourquoi(s)?)?)?;
    println!("On a de la chance : {}", s);
    Ok(())
}

#[derive(Debug)]
enum MyErrors {
    PasContent(String),
    PasALHeure(Duration),
    ParceQUE,
}

#[rustfmt::skip]
fn timeout(s: String) -> Result<String, MyErrors> {
    if random() { Ok(s) } 
    else { Err(MyErrors::PasALHeure(Duration::from_secs(1))) }
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
