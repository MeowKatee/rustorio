#![forbid(unsafe_code)]

use rustorio::{
    self, Bundle, ResourceType, Tick,
    buildings::Furnace,
    gamemodes::Standard,
    recipes::{CopperSmelting, IronSmelting},
    resources::{CopperOre, IronOre, Point},
    territory::{Miner, Territory},
};

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

const IRON_AMOUNT: u32 = 10;
const COPPER_AMOUNT: u32 = 5;

const MAX_MINER: u32 = 20;

fn wait_for_resource<O: ResourceType, const N: u32>(
    territory: &mut Territory<O>,
    tick: &mut Tick,
) -> Bundle<O, N> {
    if territory.num_miners() == 0_u32 {
        territory.hand_mine(tick)
    } else {
        while !tick.advance_until(|tick| territory.resources(tick).amount() >= N, 100) {}
        territory.resources(tick).bundle().unwrap()
    }
}

fn build_miner(
    tick: &mut Tick,
    iron_territory: &mut Territory<IronOre>,
    copper_territory: &mut Territory<CopperOre>,
    mut fur: Vec<Furnace<IronSmelting>>,
) -> (Vec<Furnace<IronSmelting>>, Miner) {
    let iron_ore = wait_for_resource::<_, IRON_AMOUNT>(iron_territory, tick);
    let copper_ore = wait_for_resource::<_, 5>(copper_territory, tick);

    let mut fur = fur.pop().unwrap();
    fur.inputs(tick).0.add(iron_ore);
    while !tick.advance_until(|tick| fur.outputs(tick).0.amount() == IRON_AMOUNT, 100) {}
    let mut iron_recipe = fur
        .outputs(tick)
        .0
        .bundle::<IRON_AMOUNT>()
        .unwrap()
        .to_resource();

    let Ok(mut fur) = fur.change_recipe(CopperSmelting) else {
        panic!()
    };

    fur.inputs(tick).0.add(copper_ore);
    while !tick.advance_until(|tick| fur.outputs(tick).0.amount() == COPPER_AMOUNT, 100) {}
    let mut copper_recipe = fur
        .outputs(tick)
        .0
        .bundle::<COPPER_AMOUNT>()
        .unwrap()
        .to_resource();

    let miner = if let Ok(copper) = copper_recipe.bundle::<5>()
        && let Ok(iron) = iron_recipe.bundle::<10>()
    {
        Miner::build(iron, copper)
    } else {
        panic!();
    };

    let Ok(fur) = fur.change_recipe(IronSmelting) else {
        panic!()
    };

    (vec![fur], miner)
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Point, 200>) {
    let StartingResources {
        iron,
        mut iron_territory,
        mut copper_territory,
        steel_technology: _,
    } = starting_resources;

    let mut fur = vec![Furnace::build(&tick, IronSmelting, iron)];

    // allocate max mines for both resource.
    for i in 0..2 * MAX_MINER {
        let (fur_, miner) = build_miner(&mut tick, &mut iron_territory, &mut copper_territory, fur);
        fur = fur_;

        // policy to allocate miners
        if i % 2 == 0 {
            _ = copper_territory.add_miner(&tick, miner);
        } else {
            _ = iron_territory.add_miner(&tick, miner);
        }
    }

    println!("Cost {} ticks to initialize miners", tick.cur());

    todo!("Return the `tick` and the victory resources to win the game!")
}
