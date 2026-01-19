#![forbid(unsafe_code)]

use std::sync::LazyLock;

use rustorio::buildings::Furnace;
use rustorio::recipes::{CopperSmelting, FurnaceRecipe, IronSmelting};
use rustorio::resources::{CopperOre, IronOre};
use rustorio::territory::{Miner, Territory};
use rustorio::{self, Bundle, Resource, ResourceType, Tick};

mod typing;
pub use typing::{ResTuple1, ResTuple2, ResTuple3, ResTuple4};

pub static MAX_MINER: LazyLock<u32> =
    LazyLock::new(|| option_env!("MAX_MINER").unwrap_or("20").parse().unwrap());
pub static MAX_FURNACE: LazyLock<usize> =
    LazyLock::new(|| option_env!("MAX_FURNACE").unwrap_or("10").parse().unwrap());

const IRON_BUILD_FUR: u32 = 10;

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

// Helper function for abstracted furnace use
fn assign_furnance<R: FurnaceRecipe>(
    fur: &mut Furnace<R>,
    tick: &Tick,
    res: Resource<<R::Inputs as ResTuple1>::RESOURCE>,
) where
    R::Inputs: ResTuple1,
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
    territory: &mut Territory<<R::Inputs as ResTuple1>::RESOURCE>,
    tick: &mut Tick,
) -> Bundle<<R::Outputs as ResTuple1>::RESOURCE, N>
where
    R::Inputs: ResTuple1,
    R::Outputs: ResTuple1,
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

/// build miner, prepare iron & copper parallelly
pub fn build_miner(
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
