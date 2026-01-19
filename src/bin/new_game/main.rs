#![forbid(unsafe_code)]

use rustorio::buildings::Furnace;
use rustorio::gamemodes::Standard;
use rustorio::recipes::IronSmelting;
use rustorio::resources::Point;
use rustorio::{Bundle, Tick};

use rustorio_game::{MAX_MINER, build_miner};

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

    println!("Cost {} ticks to initialize miners", tick.cur());

    todo!("Return the `tick` and the victory resources to win the game!")
}
