fn return_string() -> String {
    String::from("hello")
}

fn main() {
    let mut s = String::from("hello Rust");
    {
        let borrow1 = &s;
        let borrow2 = &s; // Ok pour 2 références non modifiable
        dbg!(borrow1, borrow2);
    }
    {
        let mutable_borrow = &mut s; // Ok pour 1 référence modifiable
        dbg!(mutable_borrow);
    }

    let t = return_string();

    // Aucun des blocks ci dessous ne compile, les erreurs de rustc sont mises en commentaires
    // {
    //     let mutable_borrow = &mut s; // ------ first mutable borrow occurs here
    //     let mutable_borrow2 = &mut s; // ^^^^^^ second mutable borrow occurs here
    //     dbg!(mutable_borrow, mutable_borrow2); // -------------- first borrow later used here
    // }
    // {
    //     let mutable_borrow = &mut s; // ------ mutable borrow occurs here
    //     let borrow = &s; // ^^ immutable borrow occurs here
    //     dbg!(mutable_borrow, borrow); // -------------- mutable borrow later used here
    // }
    // {
    //     let borrow = &s; // -- immutable borrow occurs here
    //     let mutable_borrow = &mut s; // ^^^^^^ mutable borrow occurs here
    //     dbg!(borrow, mutable_borrow); // ------ immutable borrow later used here
    // }
}

// // Cette fonction ne compile pas car elle retourne la référence d'une String qui sera droppée a la fin de la fonction
// fn return_string_ref<'a>() -> &'a String {
//     let s = String::from("hello");
//     &s // ^^ returns a reference to data owned by the current function
// }
