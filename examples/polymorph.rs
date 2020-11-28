fn main() {
  let baby = Child { age: 2 };
  let duck = Duck {};
  let note = MusicNotes::C;

  print_a_noise(&baby);
  print_a_noise(&duck);
  print_a_noise(&note);

  let noisy_boxes: Vec<Box<dyn Noisy>> =
    vec![Box::new(baby), Box::new(duck), Box::new(note)];

  for noisy in noisy_boxes {
    print_a_noise_from_box(&noisy);
  }
}

trait Noisy {
  fn make_a_noise(&self) -> String;
}

struct Duck {}

struct Child {
  age: u8,
}

#[rustfmt::skip]
#[derive(Debug)]
enum MusicNotes{
    A,B,C,D,E,F,G
}

// Et les implémentations

impl Noisy for Duck {
  fn make_a_noise(&self) -> String {
    String::from("Couac couac")
  }
}

impl Noisy for Child {
  fn make_a_noise(&self) -> String {
    match self.age {
      0..=1 => String::from("Ouiiinnnnnnnn!!!!"),
      2..=4 => String::from("Maaaamaaaan!!!"), // parfois papa
      5..=12 => String::from("Aïeaïeaïe!"),
      _ => String::from("I'm too old for this sh.t."),
    }
  }
}

impl Noisy for MusicNotes {
  fn make_a_noise(&self) -> String {
    format!("{:?}", self)
  }
}

fn print_a_noise<T>(noisy: &T)
where
  T: Noisy,
{
  println!("{}", noisy.make_a_noise());
}

fn print_a_noise_from_box(noisy: &Box<dyn Noisy>) {
  println!("{}", noisy.make_a_noise());
}
