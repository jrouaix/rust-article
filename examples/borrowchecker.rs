// Cette fonction ne compile pas car la valeur est `moved`
// de s1 vers s2, il n'est donc plus possible d'utiliser s1
fn a_value_owned_by_2_variables() {
  let s1 = String::from("hello");
  // -- move occurs because `s1` has type `std::string::String`,
  // which does not implement the `Copy` trait
  let s2 = s1;
  // -- value moved here
  println!("{}, world!", s1);
  //  ^^ value borrowed here after move
}

fn main() {
  let mut s = String::from("hello Rust");
  {
    let borrow1 = &s;
    let borrow2 = &s;
    // Ok pour 2 références non modifiable
    dbg!(borrow1, borrow2);
    // dbg! imprime une représentation de debug
    // de ce qui lui est passé en paramètres
    // sur la console
  }
  {
    let mutable_borrow = &mut s;
    // Ok pour 1 référence modifiable
    dbg!(mutable_borrow);
  }

  // Aucun des blocks ci dessous ne compile,
  // les erreurs de rustc sont mises en commentaires
  {
    let mutable_borrow = &mut s;
    // ------ first mutable borrow occurs here
    let mutable_borrow2 = &mut s;
    // ^^^^^^ second mutable borrow occurs here
    dbg!(mutable_borrow, mutable_borrow2);
    // -------------- first borrow later used here
  }
  {
    let mutable_borrow = &mut s;
    // ------ mutable borrow occurs here
    let borrow = &s;
    // ^^ immutable borrow occurs here
    dbg!(mutable_borrow, borrow);
    // -------------- mutable borrow later used here
  }
  {
    let borrow = &s;
    // -- immutable borrow occurs here
    let mutable_borrow = &mut s;
    // ^^^^^^ mutable borrow occurs here
    dbg!(borrow, mutable_borrow);
    // ------ immutable borrow later used here
  }
}

fn try_return_string_ref() {
  // Aucun problème, la fonction `return_string` retourne
  // une valeur et en donne la propriété a la variable t.
  let _t1 = return_string();
  // ici c'est une autre histoire (voir la fonction)
  let _t2 = return_string_ref();
}

fn return_string() -> String {
  String::from("hello")
}

// Cette fonction ne compile pas car elle retourne une
// référence d'une valeur qui sera droppée en fin de scope
fn return_string_ref<'a>() -> &'a String {
  let s = String::from("hello");
  &s
  // ^^ returns a reference to data owned
  // by the current function
}

fn borrow_checker_prevent_from_change_while_iterate() {
  let mut list = vec![1, 2, 3, 4];
  list.push(5);
  for i in 5..10 {
    // aucun problème ici
    list.push(i);
  }
  for j in &list {
    // pas possible ici parce qu'un emprunt
    // est déjà en cours
    list.push(j * j);
  }
}
