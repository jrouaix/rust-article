// version normal
fn main() {
    let v = vec![1, 2, 3];
    print!("vec = {:?} !", v);
}

// // version réellement compilée
// fn expanded_main() {
//     let v = <[_]>::into_vec(box [1, 2, 3]);
//     ::std::io::_print(::core::fmt::Arguments::new_v1(
//         &["vec = ", " !"],
//         &match (&v,) {
//             (arg0,) => [::core::fmt::ArgumentV1::new(
//                 arg0,
//                 ::core::fmt::Debug::fmt,
//             )],
//         },
//     ));
// }

// Output :
// vec = [1, 2, 3] !

// #[derive(Debug)]
// struct Test {
//     i: u32,
//     s: String,
// }

struct Test {
    i: u32,
    s: String,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Test {
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter,
    ) -> ::core::fmt::Result {
        match *self {
            Test {
                i: ref __self_0_0,
                s: ref __self_0_1,
            } => {
                let mut debug_trait_builder =
                    f.debug_struct("Test");
                let _ = debug_trait_builder
                    .field("i", &&(*__self_0_0));
                let _ = debug_trait_builder
                    .field("s", &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
