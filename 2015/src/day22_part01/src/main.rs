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

        // todo effects and attacks

        for new_state in State::get_options(&current_state) {
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

    shield_timer: Option<isize>,
    posion_timer: Option<isize>,
    recharge_timer: Option<isize>,
}

impl Actor {
    fn new(hp: isize, mana: isize, armor: isize) -> Self {
        Actor { hp: hp, mana: mana, armor: armor, shield_timer: None, posion_timer: None, recharge_timer: None }
    }
}

struct State {
    player: Actor,
    boss: Actor,
    parent: Option<(Rc<State>, Spell)>,
}

impl State {
    fn new_root(player: Actor, boss: Actor) -> Rc<Self> {
        Rc::new(State { player: player, boss: boss, parent: None })
    }

    fn new_child(parent: &Rc<Self>, creation_spell: Spell) -> Self {
        let mut new_state = State { 
            player: parent.player,
            boss: parent.boss,
            parent: Some((parent.clone(), creation_spell)),
        };
        new_state.player.mana -= creation_spell.mana_cost();
        new_state
    }

    fn apply_spell(parent: &Rc<Self>, spell: Spell) -> Option<Rc<State>> {
        let mut child = State::new_child(parent, MagicMissile);
        
        match spell {
            MagicMissile => {
                child.boss.hp -= 4;
            }
            Drain => {
                child.boss.hp -= 2;
                child.player.hp += 2;
            }
            Shield => {
                if child.player.shield_timer.is_some() {
                    return None;
                }
                child.player.shield_timer = Some(6);
                child.player.armor += 7;
            }
            Posion => {
                if child.boss.posion_timer.is_some() {
                    return None;
                }
                child.boss.posion_timer = Some(6);
            }
            Recharge => {
                if child.player.recharge_timer.is_some() {
                    return None;
                }
                child.player.recharge_timer = Some(5);
            }
        }
        Some(Rc::new(child))
    }

    fn get_options(parent: &Rc<Self>) -> Vec<Rc<Self>> {
        let mut options = vec![];
        for spell in Spell::get_all() {
            if let Some(new_state) = State::apply_spell(parent, spell) {
                options.push(new_state);
            }
        }
        options
    }

    fn is_victory(&self) -> bool {
        self.player.mana >= MIN_SPELL_COST && self.player.hp > 0 && self.boss.hp <= 0
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