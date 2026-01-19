#![forbid(unsafe_code)]

use std::sync::LazyLock;

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

const IRON_BUILD_FUR: u32 = 10;
static MAX_MINER: LazyLock<u32> =
    LazyLock::new(|| option_env!("MAX_MINER").unwrap_or("20").parse().unwrap());
static MAX_FURNACE: LazyLock<usize> =
    LazyLock::new(|| option_env!("MAX_FURNACE").unwrap_or("10").parse().unwrap());

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

fn calculate_requirement<const AMOUNT: u32>(furs_num: usize) -> Vec<u32> {
    let mut initial = vec![(AMOUNT as usize / furs_num) as u32; furs_num];
    initial
        .iter_mut()
        .take(AMOUNT as usize % furs_num)
        .for_each(|i| *i += 1);
    initial
}

fn earn_resource_parallel<R: FurnaceRecipe, const N: u32>(
    mut furs: Vec<&mut Furnace<R>>,
    territory: &mut Territory<<R::Inputs as ResTuple>::RESOURCE>,
    tick: &mut Tick,
) -> Bundle<<R::Outputs as ResTuple>::RESOURCE, N>
where
    R::Inputs: ResTuple,
    R::Outputs: ResTuple,
{
    let arangement = calculate_requirement::<N>(furs.len());
    let mut material = wait_for_resource::<_, N>(territory, tick).to_resource();

    for (fur, &res_num) in furs.iter_mut().zip(arangement.iter()) {
        let material = material.split_off(res_num).unwrap();
        assign_furnance(fur, tick, material);
    }

    while !tick.advance_until(
        |tick| {
            furs.iter_mut()
                .zip(arangement.iter())
                .all(|(fur, &arrangement)| fur.outputs(tick).access_res().amount() >= arrangement)
        },
        100,
    ) {}

    let mut res_total = Resource::new_empty();
    for (fur, &res_num) in furs.iter_mut().zip(arangement.iter()) {
        let Ok(res) = fur.outputs(tick).access_res().split_off(res_num) else {
            continue;
        };
        res_total.add(res);
    }

    res_total.bundle().unwrap()
}

// TODO:
//  - split resources
//  - parallel smelting
fn build_miner(
    tick: &mut Tick,
    iron_territory: &mut Territory<IronOre>,
    copper_territory: &mut Territory<CopperOre>,
    mut furs: Vec<Furnace<IronSmelting>>,
) -> (Miner, Vec<Furnace<IronSmelting>>) {
    while iron_territory.resources(tick).amount() >= IRON_BUILD_FUR && furs.len() < *MAX_FURNACE {
        let furs_ref = furs.iter_mut().collect::<Vec<_>>();
        let iron = earn_resource_parallel(furs_ref, iron_territory, tick);
        furs.push(Furnace::build(tick, IronSmelting, iron));
    }

    let furs_ref = furs.iter_mut().collect::<Vec<_>>();
    let iron = earn_resource_parallel(furs_ref, iron_territory, tick);

    let mut furs = furs
        .into_iter()
        .map(|fur| fur.change_recipe(CopperSmelting))
        .flatten()
        .collect::<Vec<_>>();

    let furs_ref = furs.iter_mut().collect::<Vec<_>>();
    let copper = earn_resource_parallel(furs_ref, copper_territory, tick);

    let furs = furs
        .into_iter()
        .map(|fur| fur.change_recipe(IronSmelting))
        .flatten()
        .collect::<Vec<_>>();

    (Miner::build(iron, copper), furs)
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
