use crate::entities::HasLife;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Character {
    pub health: isize,
    pub desire: isize,
    pub ambition: isize,
    pub balance: isize,
    pub unlocks: HashSet<CharacterType>,
}

impl Character {
    pub fn default() -> Self {
        Self {
            health: 100,
            desire: 0,
            ambition: 0,
            balance: 0,
            unlocks: HashSet::new(),
        }
    }

    pub fn check_unlocks(&self, target: CharacterType) -> bool {
        self.unlocks.iter().any(|el| *el == target)
    }

    pub fn unlock(&mut self, target: CharacterType) {
        self.unlocks.insert(target);
    }
}

impl HasLife for Character {
    fn take_damage(&mut self, damage: isize) {
        self.health -= damage;
    }

    fn weaken(&mut self, weaken_strength: isize) {
        self.desire -= weaken_strength;
        self.ambition -= weaken_strength;
        self.balance -= weaken_strength;
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum CharacterType {
    Wizard(WizardClass),
    Sage(SageClass),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct WizardClass {
    pub mana: isize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SageClass {
    pub guidance: isize,
}

pub trait WizardBehavior {
    fn recover(&mut self);
    fn cast_spell<T: HasLife>(&mut self, target: &mut T) -> Option<isize>;
}

impl WizardBehavior for WizardClass {
    fn recover(&mut self) {
        self.mana += 10
    }
    fn cast_spell<T: HasLife>(&mut self, target: &mut T) -> Option<isize> {
        self.mana -= 10;
        target.take_damage(10);
        Some(10)
    }
}

pub trait SageBehavior {
    fn meditate(&mut self);
    fn weaken<T: HasLife>(&mut self, target: &mut T);
}

impl SageBehavior for SageClass {
    fn meditate(&mut self) {
        self.guidance += 10
    }
    fn weaken<T: HasLife>(&mut self, target: &mut T) {
        self.guidance -= 10;
        target.weaken(10);
    }
}
