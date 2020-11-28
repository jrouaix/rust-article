// version normal
#![feature(format_args_capture)]

fn main() {
  let vector = vec![1, 2, 3];
  let integer = 42;
  print!("vec = {vector:?}, integer = {integer}");
}

// // version réellement compilée
// fn main() {
//     let v = <[_]>::into_vec(box [1, 2, 3]);
//     let integer = 42;
//     ::std::io::_print(::core::fmt::Arguments::new_v1(
//         &["vec = ", ", integer = "],
//         &match (&v, &integer) {
//             (arg0, arg1) => [
//                 ::core::fmt::ArgumentV1::new(
//                     arg0,
//                     ::core::fmt::Debug::fmt,
//                 ),
//                 ::core::fmt::ArgumentV1::new(
//                     arg1,
//                     ::core::fmt::Display::fmt,
//                 ),
//             ],
//         },
//     ));
// }

// Output :
// vec = [1, 2, 3] !

#[derive(Debug)]
struct Test {
  i: u32,
  s: String,
}

// struct Test {
//     i: u32,
//     s: String,
// }
// #[automatically_derived]
// #[allow(unused_qualifications)]
// impl ::core::fmt::Debug for Test {
//     fn fmt(
//         &self,
//         f: &mut ::core::fmt::Formatter,
//     ) -> ::core::fmt::Result {
//         match *self {
//             Test {
//                 i: ref __self_0_0,
//                 s: ref __self_0_1,
//             } => {
//                 let mut debug_trait_builder =
//                     f.debug_struct("Test");
//                 let _ = debug_trait_builder
//                     .field("i", &&(*__self_0_0));
//                 let _ = debug_trait_builder
//                     .field("s", &&(*__self_0_1));
//                 debug_trait_builder.finish()
//             }
//         }
//     }
// }
