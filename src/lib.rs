#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::sync::LazyLock;

use rustorio::buildings::Furnace;
use rustorio::recipes::{CopperSmelting, FurnaceRecipe, IronSmelting};
use rustorio::resources::{CopperOre, IronOre};
use rustorio::territory::{Miner, Territory};
use rustorio::{self, Bundle, Resource, ResourceType, Tick};

mod typing;
pub use typing::{
    ResBundleTuple1, ResBundleTuple2, ResBundleTuple3, ResBundleTuple4, ResTuple1, ResTuple2,
    ResTuple3, ResTuple4,
};

pub use crate::assembler_utils::{assemble_parallel_1to1, assemble_parallel_2to1};
pub use crate::smelting_utils::smelting_parallel_1to1;

pub static MAX_MINER: LazyLock<u32> =
    LazyLock::new(|| option_env!("MAX_MINER").unwrap_or("20").parse().unwrap());
pub static MAX_FURNACE: LazyLock<usize> =
    LazyLock::new(|| option_env!("MAX_FURNACE").unwrap_or("10").parse().unwrap());

const IRON_BUILD_FUR: u32 = 10;

mod assembler_utils;
mod smelting_utils;

pub fn mine_resource<const N: u32, O: ResourceType>(
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

/// build miner, prepare iron & copper parallelly
pub fn build_miner(
    tick: &mut Tick,
    iron_territory: &mut Territory<IronOre>,
    copper_territory: &mut Territory<CopperOre>,
    mut furs: Vec<Furnace<IronSmelting>>,
) -> (Miner, Vec<Furnace<IronSmelting>>) {
    while iron_territory.resources(tick).amount() >= IRON_BUILD_FUR && furs.len() < *MAX_FURNACE {
        let res = mine_resource(iron_territory, tick);
        let iron = smelting_parallel_1to1::<_, 10>(&mut furs, res, tick, false);
        furs.push(Furnace::build(tick, IronSmelting, iron));
    }

    let res = mine_resource(iron_territory, tick);
    let iron = smelting_parallel_1to1::<_, 10>(&mut furs, res, tick, false);

    let mut furs = change_recipe(furs, CopperSmelting);
    let res = mine_resource(copper_territory, tick);
    let copper = smelting_parallel_1to1::<_, 5>(&mut furs, res, tick, false);

    let furs = change_recipe(furs, IronSmelting);
    (Miner::build(iron, copper), furs)
}

pub fn change_recipe<Recipe>(
    furs: Vec<Furnace<impl FurnaceRecipe>>,
    new_recipe: Recipe,
) -> Vec<Furnace<Recipe>>
where
    Recipe: FurnaceRecipe + Copy,
{
    furs.into_iter()
        .flat_map(|fur| fur.change_recipe(new_recipe))
        .collect::<Vec<_>>()
}

// Helper function for abstracted furnace use
fn assign_furnance<R: FurnaceRecipe>(
    fur: &mut Furnace<R>,
    tick: &Tick,
    res: Resource<<R::Inputs as ResTuple1>::ResType>,
) where
    R::Inputs: ResTuple1,
{
    fur.inputs(tick).access_res().add(res);
}

fn calculate_requirement<const AMOUNT: u32>(machine_num: usize) -> Vec<u32> {
    let mut initial = vec![(AMOUNT as usize / machine_num) as u32; machine_num];
    initial
        .iter_mut()
        .take(AMOUNT as usize % machine_num)
        .for_each(|i| *i += 1);
    initial
}
