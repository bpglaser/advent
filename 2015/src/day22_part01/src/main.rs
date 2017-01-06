use std::env::args;
use std::rc::Rc;

use Spell::*;

const MIN_SPELL_COST: isize = 53;

fn main() {
    let boss = get_boss();
    let player = Actor::new(50, 500, 0);

    let spell_path = find_best_spell_path(player, boss);
    let mana: isize = spell_path.iter().map(|spell| spell.mana_cost() ).sum();

    println!("Answer: {}", mana);
}

fn find_best_spell_path(player: Actor, boss: Actor) -> Vec<Spell> {
    let initial_state = State::new_root(player, boss);
    let mut stack = vec![initial_state];

    while stack.len() > 0 {
        let current_state = stack.remove(0);

        for new_state in current_state.get_options() {
            if new_state.is_victory() {
                return new_state.get_spell_path();
            }
            stack.push(new_state);
        }
    }

    panic!("Unable to find spell path!");
}

#[derive(Copy, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Posion,
    Recharge,
}

impl Spell {
    fn get_all() -> Vec<Spell> {
        vec![MagicMissile, Drain, Shield, Posion, Recharge]
    }

    fn mana_cost(&self) -> isize {
        match self {
            &MagicMissile => 53,
            &Drain => 73,
            &Shield => 113,
            &Posion => 173,
            &Recharge => 229,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Actor {
    hp: isize,
    mana: isize,
    armor: isize, // Armor is ignored for spells
}

impl Actor {
    fn new(hp: isize, mana: isize, armor: isize) -> Self {
        Actor { hp: hp, mana: mana, armor: armor }
    }
}

struct State {
    player: Actor,
    boss: Actor,
    parent: Option<(Rc<State>, Spell)>,
}

impl State {
    fn new_root(player: Actor, boss: Actor) -> Self {
        State { player: player, boss: boss, parent: None }
    }

    fn new_child(&self, creation_spell: Spell) -> Self {
        State { player: self.player, boss: self.boss, parent: Some((Rc::new(self), creation_spell)) }
    }

    fn apply_spell(&self, spell: Spell) -> Option<State> {
        match spell {
            MagicMissile => {
                let mut child = self.new_child(MagicMissile);
            }
            // Drain,
            // Shield,
            // Posion,
            // Recharge,
            _ => unimplemented!()
        }
        unimplemented!()
    }

    fn is_victory(&self) -> bool {
        self.player.mana >= MIN_SPELL_COST && self.player.hp > 0 && self.boss.hp <= 0
    }

    fn get_options(&self) -> Vec<Self> {
        let mut options = vec![];
        for spell in Spell::get_all() {
            if let Some(new_state) = self.apply_spell(spell) {
                options.push(new_state);
            }
        }
        options
    }

    fn get_spell_path(&self) -> Vec<Spell> {
        let mut spell_path = vec![];
        
        let mut working_state = self;

        while let Some((ref parent_state, ref creation_spell)) = working_state.parent {
            spell_path.push(*creation_spell);
            working_state = parent_state;
        }

        spell_path.reverse();

        spell_path
    }
}

fn get_boss() -> Actor {
    let args: Vec<isize> = args().skip(1).map(|s| s.parse().expect("Input must be number")).collect();
    assert_eq!(2, args.len(), "Invalid number of args");
    Actor::new(args[0], 0, args[1])
}