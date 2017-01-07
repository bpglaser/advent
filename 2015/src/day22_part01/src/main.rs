use std::error::Error;
use std::env::args;

use GameSignal::*;
use Spell::*;

fn main() {
    let (player, boss) = get_input().expect("Invalid input");

    let spell_path = find_best_spell_path(player, boss);
    let mana: usize = spell_path.iter().map(|spell| spell.mana_cost() ).sum();

    println!("Optimal spell path found:");

    for spell in spell_path {
        println!("{:?}", spell);
    }

    println!("Answer: {}", mana);
}

fn find_best_spell_path(player: Actor, boss: Actor) -> Vec<Spell> {
    let initial_state = State::new_root(player, boss);
    let mut stack = vec![initial_state];

    while stack.len() > 0 {
        let current_state = stack.remove(0);

        // ==== Evaluate Effects ====

        let current_state = match current_state.evaluate_effects() {
            Ok(state) => state,
            Err(BossDeath(spell_path)) => return spell_path,
            _ => unreachable!(),
        };

        // ==== Cast Spell ====

        for new_state in current_state.get_options() {

            let new_state = match new_state {
                Ok(state) => state,
                Err(BossDeath(spell_path)) =>  return spell_path,
                Err(OutOfMana) => continue,
                e @ _ => panic!("NYE: {:?}", e), // todo handle
            };

            // ==== Evaluate Effects ====
            
            let new_state = match new_state.evaluate_effects() {
                Ok(state) => state,
                Err(BossDeath(spell_path)) => return spell_path,
                _ => unreachable!(),
            };

            // ==== Boss Attack ====

            let new_state = match new_state.attack_player() {
                Ok(state) => state,
                Err(PlayerDeath) => continue,
                Err(_) => unreachable!(),
            };

            stack.push(new_state);
        }
    }

    panic!("Unable to find spell path!");
}

#[derive(Debug)]
enum GameSignal {
    BossDeath(Vec<Spell>),
    OutOfMana,
    PlayerDeath,
}

#[derive(Copy, Clone, Debug)]
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

    fn mana_cost(&self) -> usize {
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
    hp: usize,
    mana: usize,
    attack: usize, // Only boss can attack
    armor: usize, // Armor is ignored for spells
}

impl Actor {
    fn new(hp: usize, mana: usize, attack: usize) -> Self {
        Actor { 
            hp: hp,
            mana: mana,
            attack: attack,
            armor: 0,
        }
    }

    fn new_player(hp: usize, mana: usize) -> Self {
        Actor::new(hp, mana, 0)
    }

    fn new_boss(hp: usize, attack: usize) -> Self {
        Actor::new(hp, 0, attack)
    }
}

#[derive(Clone, Debug)]
struct State {
    player: Actor,
    boss: Actor,
    parent: Option<(Box<State>, Spell)>,

    shield_timer: usize,
    posion_timer: usize,
    recharge_timer: usize,
}

impl State {
    fn new_root(player: Actor, boss: Actor) -> Self {
        State {
            player: player,
            boss: boss,
            parent: None,
            shield_timer: 0,
            posion_timer: 0,
            recharge_timer: 0,
        }
    }

    fn new_child(&self, creation_spell: Spell) -> Self {
        let clone = self.clone();
        State { 
            player: self.player,
            boss: self.boss,
            parent: Some((Box::new(clone), creation_spell)),
            shield_timer: self.shield_timer,
            posion_timer: self.posion_timer,
            recharge_timer: self.recharge_timer,
        }
    }

    fn apply_spell(&self, spell: Spell) -> Result<Self, GameSignal> {
        let mut child = self.new_child(spell);
        if child.player.mana < spell.mana_cost() {
            return Err(OutOfMana);
        }
        child.player.mana -= spell.mana_cost();
        
        match spell {
            MagicMissile => {
                if child.boss.hp <= 4 {
                    return Err(BossDeath(child.get_spell_path()));
                }
                child.boss.hp -= 4;
            }
            Drain => {
                if child.boss.hp <= 2 {
                    return Err(BossDeath(child.get_spell_path()));
                }
                child.boss.hp -= 2;
                child.player.hp += 2;
            }
            Shield => {
                if child.shield_timer == 0  {
                    child.shield_timer = 6;
                    child.player.armor += 7;
                }
            }
            Posion => {
                if child.posion_timer == 0 {
                    child.posion_timer = 6;
                }
            }
            Recharge => {
                if child.recharge_timer == 0 {
                    child.recharge_timer = 5;
                }
            }
        }

        Ok(child)
    }

    fn get_options(&self) -> Vec<Result<Self, GameSignal>> {
        let mut options = vec![];
        for spell in Spell::get_all() {
            options.push(self.apply_spell(spell));
        }
        options
    }

    fn evaluate_effects(&self) -> Result<Self, GameSignal> {
        let mut state = self.clone();

        // Shield effect
        if state.shield_timer > 0 {
            state.shield_timer -= 1;
            if state.shield_timer == 0 {
                state.player.armor = 0;
            }
        }
        
        // Posion effect
        if state.posion_timer > 0 {
            state.posion_timer -= 1;
            if state.boss.hp <= 3 {
                state.boss.hp = 0;
                return Err(BossDeath(self.get_spell_path()));
            }
            state.boss.hp -= 3;
        }

        // Recharge effect
        if state.recharge_timer > 0 {
            state.recharge_timer -= 1;
            state.player.mana += 101;
        }

        Ok(state)
    }

    fn attack_player(&self) -> Result<Self, GameSignal> {
        let mut state = self.clone();

        let attack_amount;
        if state.boss.attack <= state.player.armor {
            attack_amount = 1; // Attack does a minimum of 1 damage
        } else {
            attack_amount = state.boss.attack - state.player.armor;
        }

        if state.player.hp <= attack_amount {
            return Err(GameSignal::PlayerDeath);
        }
        state.player.hp -= attack_amount;

        Ok(state)
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

fn get_input() -> Result<(Actor, Actor), Box<Error>> {
    let args: Vec<_> = args().skip(1).collect();
    assert_eq!(4, args.len(), "Invalid number of args");

    let player = Actor::new_player(args[0].parse()?, args[1].parse()?);
    let boss = Actor::new_boss(args[2].parse()?, args[3].parse()?);

    println!("Input player: {:?}", player);
    println!("Input boss: {:?}", boss);


    Ok((player, boss))
}