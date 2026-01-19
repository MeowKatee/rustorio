#![forbid(unsafe_code)]

use rustorio::buildings::{Assembler, Furnace};
use rustorio::gamemodes::Standard;
use rustorio::recipes::{CopperSmelting, CopperWireRecipe, IronSmelting};
use rustorio::resources::{Copper, Iron, Point};
use rustorio::{Bundle, HandRecipe, Resource, Tick};

use rustorio_game::{
    MAX_FURNACE, MAX_MINER, assemble_parallel_1to1, build_miner, change_recipe, mine_resource,
    smelting_parallel_1to1,
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

    let iron_ore = mine_resource(&mut iron_territory, &mut tick);

    const ASSEMBLER_NUM: u32 = 100;

    let mut iron: Resource<Iron> = smelting_parallel_1to1::<_, { 1000 + 6 * ASSEMBLER_NUM }>(
        &mut furs, iron_ore, &mut tick, true,
    )
    .to_resource();

    let mut furs = change_recipe(furs, CopperSmelting);

    let copper_ore = mine_resource(&mut copper_territory, &mut tick);
    let mut copper = smelting_parallel_1to1::<_, { 800 + 6 * ASSEMBLER_NUM }>(
        &mut furs, copper_ore, &mut tick, true,
    )
    .to_resource();

    let mut assems = vec![];

    let time = tick.cur();
    for _ in 0..ASSEMBLER_NUM {
        let iron = iron.bundle().unwrap();
        let copper = copper.bundle().unwrap();

        if assems.is_empty() {
            assems.push(bootstrap_assembler(iron, copper, &mut tick));
        } else {
            let copper_wires = assemble_parallel_1to1::<_, 6>(
                &mut assems,
                copper.to_resource().bundle().unwrap(),
                &mut tick,
            )
            .to_resource()
            .bundle()
            .unwrap();

            assems.push(Assembler::build(
                &tick,
                CopperWireRecipe,
                copper_wires,
                iron,
            ));
        }
    }

    println!(
        "Cost {} ticks to bootstrap {ASSEMBLER_NUM} assemblers",
        tick.cur() - time
    );

    todo!("Return the `tick` and the victory resources to win the game!")
}

fn bootstrap_assembler(
    iron: Bundle<Iron, 6>,
    copper: Bundle<Copper, 12>,
    tick: &mut Tick,
) -> Assembler<CopperWireRecipe> {
    let mut coppers = copper.to_resource();
    let mut copper_wires = Resource::new_empty();
    let time = tick.cur();
    while let Ok(copper) = coppers.bundle() {
        copper_wires.add(
            <CopperWireRecipe as HandRecipe>::craft(tick, (copper,))
                .0
                .to_resource(),
        );
    }

    println!(
        "Cost {} ticks to bootstrap first assember",
        tick.cur() - time
    );

    Assembler::build(tick, CopperWireRecipe, copper_wires.bundle().unwrap(), iron)
}
