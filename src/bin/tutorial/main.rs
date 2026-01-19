#![forbid(unsafe_code)]

use rustorio::buildings::Furnace;
use rustorio::gamemodes::Tutorial;
use rustorio::recipes::CopperSmelting;
use rustorio::resources::Copper;
use rustorio::{self, Bundle, Tick};

type GameMode = Tutorial;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

fn user_main(mut tick: Tick, starting_resources: StartingResources) -> (Tick, Bundle<Copper, 4>) {
    tick.log(true);

    let StartingResources {
        iron,
        mut copper_territory,
        iron_territory: _,
        guide: _,
    } = starting_resources;

    let copper = copper_territory.hand_mine::<4>(&mut tick);

    // To start, run the game using `rustorio play tutorial` (or whatever this save is called), and follow the hint.
    // If you get stuck, try giving the guide other objects you've found, like the `tick` object.
    let mut fur = Furnace::build(&tick, CopperSmelting, iron);
    fur.inputs(&tick).0.add(copper);
    while !tick.advance_until(|tick| fur.outputs(tick).0.amount() == 4, 10) {}
    let copper = fur.outputs(&tick).0.bundle::<4>().unwrap();
    (tick, copper)
}
