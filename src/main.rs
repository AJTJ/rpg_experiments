use std::collections::HashSet;

use rpg::{
    character::{self, Character, CharacterType, WizardClass},
    entities::Troll,
};

fn main() {
    let mut character = Character::default();
    let mut troll = Troll::default();

    troll.attack(&mut character);

    character.unlock(CharacterType::Wizard(WizardClass { mana: 100 }));

    let second_spell = character
        .unlocks
        .get(&CharacterType::Wizard(WizardClass { mana: 100 }));

    println!("{:?}", troll);

    println!("{:?}", character);
}
