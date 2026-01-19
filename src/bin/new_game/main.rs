#![forbid(unsafe_code)]

use rustorio::buildings::{Assembler, Furnace};
use rustorio::gamemodes::Standard;
use rustorio::recipes::{CopperSmelting, CopperWireRecipe, ElectronicCircuitRecipe, IronSmelting};
use rustorio::resources::{CopperOre, IronOre, Point};
use rustorio::territory::Territory;
use rustorio::{Bundle, HandRecipe, Resource, Tick};

use rustorio_game::{
    MAX_FURNACE, MAX_MINER, build_miner, change_recipe, mine_resource, smelting_parallel_1to1,
};

type GameMode = Standard;
type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Point, 200>) {
    let StartingResources {
        iron,
        mut iron_territory,
        mut copper_territory,
        steel_technology: _,
    } = starting_resources;

    let mut furs = vec![Furnace::build(&tick, IronSmelting, iron)];

    // allocate max mines for both resource.
    let max_miner = *MAX_MINER;
    let max_furnace = *MAX_FURNACE;

    assert!(max_miner <= copper_territory.max_miners());
    assert!(max_miner <= iron_territory.max_miners());

    for i in 0..2 * max_miner {
        let (miner, furs_) =
            build_miner(&mut tick, &mut iron_territory, &mut copper_territory, furs);
        furs = furs_;

        // policy to allocate miners
        if i % 2 == 0 {
            copper_territory.add_miner(&tick, miner).unwrap();
        } else {
            iron_territory.add_miner(&tick, miner).unwrap();
        }
    }
    let time = tick.cur();
    println!("Cost {time} ticks to initialize {max_miner} miners and {max_furnace} furnances");

    let (_furs, assembler) =
        bootstrap_assembler(&mut iron_territory, &mut copper_territory, furs, &mut tick);
    let _assem = assembler.change_recipe(ElectronicCircuitRecipe).unwrap();

    todo!("Return the `tick` and the victory resources to win the game!")
}

fn bootstrap_assembler(
    iron_territory: &mut Territory<IronOre>,
    copper_territory: &mut Territory<CopperOre>,
    mut furs: Vec<Furnace<IronSmelting>>,
    tick: &mut Tick,
) -> (Vec<Furnace<CopperSmelting>>, Assembler<CopperWireRecipe>) {
    let iron_ore = mine_resource(iron_territory, tick);
    let iron = smelting_parallel_1to1(&mut furs, iron_ore, tick, true);

    let copper_ore = mine_resource::<6, _>(copper_territory, tick);
    let mut furs = change_recipe(furs, CopperSmelting);
    let mut copper = smelting_parallel_1to1::<CopperSmelting, 6>(&mut furs, copper_ore, tick, true)
        .to_resource();

    let mut copper_wires = Resource::new_empty();
    let time = tick.cur();
    while let Ok(copper) = copper.bundle() {
        copper_wires.add(
            <CopperWireRecipe as HandRecipe>::craft(tick, (copper,))
                .0
                .to_resource(),
        );
    }

    println!("Cost {} ticks to craft wires", tick.cur() - time);

    let assem = Assembler::build(
        &tick,
        CopperWireRecipe,
        copper_wires.bundle().unwrap(),
        iron,
    );

    (furs, assem)
}
