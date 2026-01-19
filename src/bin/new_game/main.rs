#![forbid(unsafe_code)]

use rustorio::{
    self, Bundle, Resource, ResourceType, Tick,
    buildings::Furnace,
    gamemodes::Standard,
    recipes::{CopperSmelting, FurnaceRecipe, IronSmelting},
    resources::{CopperOre, IronOre, Point},
    territory::{Miner, Territory},
};

type GameMode = Standard;

type StartingResources = <GameMode as rustorio::GameMode>::StartingResources;

fn main() {
    rustorio::play::<GameMode>(user_main);
}

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

trait ResTuple {
    type RESOURCE: ResourceType;
    fn access_res(&mut self) -> &mut Resource<Self::RESOURCE>;
}

impl<R: ResourceType> ResTuple for (Resource<R>,) {
    type RESOURCE = R;
    fn access_res(&mut self) -> &mut Resource<Self::RESOURCE> {
        &mut self.0
    }
}

// Helper function for abstracted furnace use
fn assign_furnance<R: FurnaceRecipe>(
    fur: &mut Furnace<R>,
    tick: &Tick,
    res: Resource<<R::Inputs as ResTuple>::RESOURCE>,
) where
    R::Inputs: ResTuple,
{
    fur.inputs(&tick).access_res().add(res);
}

fn wait_output<R: FurnaceRecipe, const N: u32>(
    fur: &mut Furnace<R>,
    tick: &mut Tick,
) -> Bundle<<R::Outputs as ResTuple>::RESOURCE, N>
where
    R::Outputs: ResTuple,
{
    while !tick.advance_until(|tick| fur.outputs(tick).access_res().amount() >= N, 100) {}
    fur.outputs(&tick).access_res().bundle().unwrap()
}

fn earn_resource<R: FurnaceRecipe, const N: u32>(
    fur: &mut Furnace<R>,
    territory: &mut Territory<<R::Inputs as ResTuple>::RESOURCE>,
    tick: &mut Tick,
) -> Bundle<<R::Outputs as ResTuple>::RESOURCE, N>
where
    R::Inputs: ResTuple,
    R::Outputs: ResTuple,
{
    let material = wait_for_resource::<_, N>(territory, tick);
    assign_furnance(fur, tick, material.to_resource());
    wait_output(fur, tick)
}

// TODO:
//  - split resources
//  - parallel smelting
fn build_miner(
    tick: &mut Tick,
    iron_territory: &mut Territory<IronOre>,
    copper_territory: &mut Territory<CopperOre>,
    mut fur: Vec<Furnace<IronSmelting>>,
) -> (Vec<Furnace<IronSmelting>>, Miner) {
    let mut fur = fur.pop().unwrap();

    let iron = earn_resource(&mut fur, iron_territory, tick);

    let Ok(mut fur) = fur.change_recipe(CopperSmelting) else {
        panic!()
    };

    let copper = earn_resource(&mut fur, copper_territory, tick);

    let Ok(fur) = fur.change_recipe(IronSmelting) else {
        panic!()
    };

    (vec![fur], Miner::build(iron, copper))
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
