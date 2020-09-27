fn main() {
    let variable = 42; // type inféré : i32
    let variable2: u8; // type précisé : u8
    let reference = &variable;
    let somme = *reference + 2;

    let mut variable_modifiable;
    variable_modifiable = "hello";
    variable_modifiable = "world";

    let ref_modifiable = &mut variable_modifiable;
    // modification de variable_modifiable
    *ref_modifiable = "hello world";
    dbg!(&ref_modifiable);
    // affiche &ref_modifiable = "hello world"
    drop(ref_modifiable);
    // suppression de la référence
    // (pas de la variable !)
    dbg!(variable_modifiable);
    // affiche variable_modifiable = "hello world"
}

fn fonction_vide() {}

fn fonction_qui_retourne_un_entier() -> i32 {
    let variable_pas_utilisee = {
        // bloc de code qui retourne un resultat
        let a = 2;
        let b = 2;
        a + b
    };
    42 // dernière ligne sans ; = return automatique
}

fn fonction_generique<T: std::fmt::Debug>(parametre: T) {}

struct UneStruct {
    champ_entier: i32,
    champ_chaine: String,
}

// rust sépare les définitions des implémentations
impl UneStruct {
    fn fonction_statique() -> UneStruct {
        // construction de une_struct
        UneStruct {
            champ_entier: 0,
            champ_chaine: String::new(),
        }
    }
    fn fonction_de_lecture(&self) -> i32 {
        self.champ_entier
    }
    fn fonction_de_mutation(&mut self, parametre: i32) {
        self.champ_entier = parametre
    }
}

enum UnEnum {
    A,            // valeur A
    B(String),    // valeur B contenant une chaine UTF-8
    C(bool, u64), // valeur C contenant un bool et un entier
}

trait UnTrait {
    fn fonction_statique_du_trait(p: UnEnum) -> String;
    fn fonction_d_instance(&mut self, parametre: bool);
}

impl UnTrait for UneStruct {
    fn fonction_statique_du_trait(un_enum: UnEnum) -> String {
        // pattern matching
        let voila = match un_enum {
            UnEnum::A => String::from("A"),
            UnEnum::B(s) => s,
            UnEnum::C(true, u) => format!("non signé {}", u),
            _ => String::from("tout les autres cas"),
        };
        format!("'{}' reçu par UneStruct", voila)
    }

    fn fonction_d_instance(&mut self, parametre: bool) {
        if parametre {
            self.fonction_de_mutation(0);
        } else {
            self.champ_chaine = String::new();
        }
    }
}
