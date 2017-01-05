extern crate itertools;

use itertools::Itertools;

use std::env::args;

use Action::*;

const PLAYER_HP: isize = 100;

fn main() {
    let boss = get_boss();
    
    let weapons = build_weapons();
    let armors = build_armors();
    let rings = build_rings();

    let mut all_item_builds = vec![];

    for weapon in &weapons {
        all_item_builds.push(vec![weapon]);

        for armor in &armors {
            all_item_builds.push(vec![weapon, armor]);

            for ring in &rings {
                all_item_builds.push(vec![weapon, armor, ring]);
            }

            for (ring1, ring2) in rings.iter().tuple_combinations() {
                all_item_builds.push(vec![weapon, armor, ring1, ring2]);
            }
        }

        for ring in &rings {
            all_item_builds.push(vec![weapon, ring]);
        }

        for (ring1, ring2) in rings.iter().tuple_combinations() {
            all_item_builds.push(vec![weapon, ring1, ring2]);
        }
    }

    let mut best_item_build = None;
    let mut best_cost = usize::min_value();

    for item_build in &all_item_builds {
        let player = Actor::from_item_build(PLAYER_HP, &item_build);
        if !player.wins_against(&boss) {
            let cost = sum_cost(item_build);
            if cost > best_cost {
                best_cost = cost;
                best_item_build = Some(item_build);
            }
        }
    }

    println!("Winning build: {:?}", best_item_build.expect("Unable to find best build"));
    println!("Total cost: {}", best_cost);
}

fn sum_cost(items: &[&Item]) -> usize {
    items.iter().map(|item| item.cost).sum()
}

fn build_weapons() -> Vec<Item> {
    vec![
        Item::weapon("Dagger", 8, 4),
        Item::weapon("Shortsword", 10, 5),
        Item::weapon("Warhammer", 25, 6),
        Item::weapon("Longsword", 40, 7),
        Item::weapon("Greataxe", 74, 8),
    ]
}

fn build_armors() -> Vec<Item> {
    vec![
        Item::armor("Leather", 13, 1),
        Item::armor("Chainmail", 31, 2),
        Item::armor("Splintmail", 53, 3),
        Item::armor("Bandedmail", 75, 4),
        Item::armor("Platemail", 102, 5),
    ]
}

fn build_rings() -> Vec<Item> {
    vec![
        Item::weapon("Damage +1", 25, 1),
        Item::weapon("Damage +2", 50, 2),
        Item::weapon("Damage +3", 100, 3),
        Item::armor("Defense +1", 20, 1),
        Item::armor("Defense +2", 40, 2),
        Item::armor("Defense +3", 80, 3),
    ]
}

#[derive(Debug)]
enum Action {
    Armor(usize),
    Damage(usize),
}

#[derive(Debug)]
struct Item {
    name: String,
    cost: usize,
    action: Action,
}

impl Item {
    fn armor(name: &str, cost: usize, armor: usize) -> Self {
        Item { name: name.to_owned(), cost: cost, action: Armor(armor) }
    }

    fn weapon(name: &str, cost: usize, damage: usize) -> Self {
        Item { name: name.to_owned(), cost: cost, action: Damage(damage) }
    }
}

#[derive(Debug)]
struct Actor {
    hp: isize,
    damage: usize,
    armor: usize,
}

impl Actor {
    fn new(hp: isize, damage: usize, armor: usize) -> Self {
        Actor { hp: hp, damage: damage, armor: armor }
    }

    fn from_item_build(hp: isize, item_build: &[&Item]) -> Self {
        let mut actor = Actor::new(hp, 0, 0);
        for item in item_build {
            match item.action {
                Armor(n) => actor.armor += n,
                Damage(n) => actor.damage += n,
            }
        }
        actor
    }

    // Simulates a fight between 2 actors, returns true if self wins
    fn wins_against(&self, other: &Self) -> bool {
        let mut self_hp = self.hp;
        let mut other_hp = other.hp;
        loop {
            let new_other_hp = other_hp - self.damage.checked_sub(other.armor).unwrap_or(0) as isize;
            if new_other_hp <= 0 {
                return true;
            }
            other_hp = new_other_hp;

            let new_self_hp = self_hp - other.damage.checked_sub(self.armor).unwrap_or(0) as isize;
            if new_self_hp <= 0 {
                return false;
            }
            self_hp = new_self_hp;
        }
    }
}

fn get_boss() -> Actor {
    let args: Vec<_> = args().skip(1).collect();
    if args.len() != 3 {
        panic!("Invalid number of args");
    }
    Actor::new(args[0].parse().unwrap(), args[1].parse().unwrap(), args[2].parse().unwrap())
}