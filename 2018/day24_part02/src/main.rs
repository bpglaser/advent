use std::cell::Cell;
use std::env::args;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use crate::AttackType::*;
use crate::GroupType::*;

fn main() {
    let path = args().nth(1).unwrap();
    let groups = read_input(&path);

    for i in 0.. {
        let mut groups = groups.clone();

        for group in &mut groups {
            if group.group_type == ImmuneSystem {
                group.attack_damage += i;
            }
        }

        match run(&mut groups) {
            ImmuneSystem => {
                print_result(groups);
                break;
            }
            Infection => {
                continue;
            }
        }
    }
}

fn run(groups: &mut [Group]) -> GroupType {
    loop {
        if !step(groups) {
            // stalemate
            return Infection;
        }
        match get_winner(&groups) {
            Some(winner) => return winner,
            None => continue,
        }
    }
}

fn print_result(groups: Vec<Group>) {
    let immune_system: Vec<_> = groups
        .iter()
        .filter(|group| group.group_type == ImmuneSystem)
        .collect();

    let infection: Vec<_> = groups
        .iter()
        .filter(|group| group.group_type == Infection)
        .collect();

    println!("==============\nFinal Results:\n==============\nImmune System:");
    if immune_system.iter().all(|group| group.units.get() == 0) {
        println!("No groups remain.");
    } else {
        for (i, group) in immune_system.iter().enumerate() {
            println!("Group {} contains {} units", i + 1, group.units.get());
        }
    }

    println!("\nInfection:");
    if infection.iter().all(|group| group.units.get() == 0) {
        println!("No groups remain.");
    } else {
        for (i, group) in infection.iter().enumerate() {
            println!("Group {} contains {} units", i + 1, group.units.get());
        }
    }

    let count = immune_system
        .iter()
        .chain(infection.iter())
        .map(|group| group.units.get())
        .sum::<u32>();
    println!("\nWinning count: {}", count);
}

fn get_winner(groups: &[Group]) -> Option<GroupType> {
    let has_immune_system = groups
        .iter()
        .filter(|group| group.group_type == ImmuneSystem)
        .any(|group| group.units.get() > 0);

    let has_infection = groups
        .iter()
        .filter(|group| group.group_type == Infection)
        .any(|group| group.units.get() > 0);

    match (has_immune_system, has_infection) {
        (true, true) => None,
        (true, false) => Some(ImmuneSystem),
        (false, true) => Some(Infection),
        (false, false) => panic!("HOW COULD THIS HAPPEN? LOL"),
    }
}

fn step(groups: &[Group]) -> bool {
    for group in groups.iter() {
        group.selected.set(false);
    }

    // Phase 1

    let mut targeters: Vec<_> = groups.iter().collect();
    targeters.sort_by(|a, b| {
        b.effective_power()
            .cmp(&a.effective_power())
            .then(b.initiative.cmp(&a.initiative))
    });

    let mut targeting = vec![];
    for attacker in targeters {
        if let Some(target) = attacker.select_target(groups) {
            targeting.push((attacker, target));
        }
    }

    // Phase 2

    targeting.sort_by_key(|(group, _)| group.initiative);

    let mut damage_done = false;
    for (attacker, target) in targeting.into_iter().rev() {
        if attacker.attack(target) {
            damage_done = true;
        }
    }

    damage_done
}

#[derive(Clone, Debug)]
struct Group {
    units: Cell<u32>,
    hit_points: u32,
    attack_damage: u32,
    attack_type: AttackType,
    initiative: u32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
    group_type: GroupType,
    selected: Cell<bool>,
    debug_id: usize,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units.get() * self.attack_damage
    }

    fn attack(&self, target: &Self) -> bool {
        if self.units.get() == 0 || target.units.get() == 0 {
            return false;
        }

        let dmg = self.potential_damage(target) / target.hit_points;

        if dmg == 0 {
            return false;
        }

        target.units.set(
            target.units.get().checked_sub(dmg).unwrap_or(0)
        );

        true
    }

    fn potential_damage(&self, target: &Self) -> u32 {
        if target.weaknesses.contains(&self.attack_type) {
            return self.effective_power() * 2;
        }
        if target.immunities.contains(&self.attack_type) {
            return 0;
        }
        self.effective_power()
    }

    fn select_target<'a>(&self, targets: &'a [Self]) -> Option<&'a Self> {
        // You can't attack if you're dead!
        if self.units.get() == 0 {
            return None;
        }

        let target = targets
            .iter()
            .filter(|target| target.units.get() > 0)
            .filter(|target| self.group_type != target.group_type)
            .filter(|target| !target.selected.get())
            .filter(|target| self.potential_damage(target) > 0)
            .max_by(|a, b| {
                self.potential_damage(a)
                    .cmp(&self.potential_damage(b))
                    .then(a.effective_power().cmp(&b.effective_power()))
                    .then(a.initiative.cmp(&b.initiative))
            });

        if let Some(target) = target {
            target.selected.set(true);
        }

        target
    }
}

#[derive(Clone, Eq, Debug, PartialEq)]
enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

#[derive(Clone, Eq, Debug, PartialEq)]
enum GroupType {
    ImmuneSystem,
    Infection,
}

fn read_input(path: &str) -> Vec<Group> {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut groups: Vec<Group> = buf.lines().filter_map(|line| line.parse().ok()).collect();

    let i = buf.lines().position(|line| line.is_empty()).unwrap();
    let mut infection = groups.split_off(i - 1);
    let mut immune_system = groups;

    for (i, group) in immune_system.iter_mut().enumerate() {
        group.group_type = ImmuneSystem;
        group.debug_id = i + 1;
    }

    for (i, group) in infection.iter_mut().enumerate() {
        group.group_type = Infection;
        group.debug_id = i + 1;
    }

    immune_system.extend(infection);

    immune_system
}

impl FromStr for Group {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("cannot parse empty string")?;
        }

        let words: Vec<_> = s.split_whitespace().collect();
        let units = Cell::new(words[0].parse().map_err(|_| "error parsing units")?);

        let i = words
            .iter()
            .position(|s| s == &"hit")
            .ok_or("cannot find hit")?;
        let hit_points = words[i - 1]
            .parse()
            .map_err(|_| "error parsing hit_points")?;

        let i = words
            .iter()
            .position(|s| s == &"damage")
            .ok_or("cannot find damage")?;
        let attack_damage = words[i - 2]
            .parse()
            .map_err(|_| "error parsing attack_damage")?;
        let attack_type = words[i - 1].parse()?;

        let initiative = words
            .last()
            .unwrap()
            .parse()
            .map_err(|_| "error parsing initiative")?;

        let mut weaknesses = vec![];
        if let Some(i) = words.iter().position(|s| s == &"weak" || s == &"(weak") {
            let mut i = i + 2;
            loop {
                match &words[i][words[i].len() - 1..] {
                    "," => {
                        weaknesses.push(words[i][..words[i].len() - 1].parse().unwrap());
                        i += 1;
                    }
                    ")" | ";" => {
                        weaknesses.push(words[i][..words[i].len() - 1].parse().unwrap());
                        break;
                    }
                    _ => {
                        weaknesses.push(words[i].parse().unwrap());
                        break;
                    }
                }
            }
        }

        let mut immunities = vec![];
        if let Some(i) = words.iter().position(|s| s == &"immune" || s == &"(immune") {
            let mut i = i + 2;
            loop {
                match &words[i][words[i].len() - 1..] {
                    "," => {
                        immunities.push(words[i][..words[i].len() - 1].parse().unwrap());
                        i += 1;
                    }
                    ")" | ";" => {
                        immunities.push(words[i][..words[i].len() - 1].parse().unwrap());
                        break;
                    }
                    _ => {
                        immunities.push(words[i].parse().unwrap());
                        break;
                    }
                }
            }
        }

        Ok(Self {
            units,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
            group_type: ImmuneSystem,
            selected: Cell::new(false),
            debug_id: 0,
        })
    }
}

impl FromStr for AttackType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bludgeoning" => Bludgeoning,
            "cold" => Cold,
            "fire" => Fire,
            "radiation" => Radiation,
            "slashing" => Slashing,
            _ => return Err(format!("unknown attack type: {}", s)),
        })
    }
}
