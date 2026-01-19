#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::sync::LazyLock;

use rustorio::buildings::{Assembler, Furnace};
use rustorio::recipes::{AssemblerRecipe, CopperSmelting, FurnaceRecipe, IronSmelting};
use rustorio::resources::{CopperOre, IronOre};
use rustorio::territory::{Miner, Territory};
use rustorio::{self, Bundle, Resource, ResourceType, Tick};

mod typing;
pub use typing::{
    ResBundleTuple1, ResBundleTuple2, ResBundleTuple3, ResBundleTuple4, ResTuple1, ResTuple2,
    ResTuple3, ResTuple4,
};

pub static MAX_MINER: LazyLock<u32> =
    LazyLock::new(|| option_env!("MAX_MINER").unwrap_or("20").parse().unwrap());
pub static MAX_FURNACE: LazyLock<usize> =
    LazyLock::new(|| option_env!("MAX_FURNACE").unwrap_or("10").parse().unwrap());

const IRON_BUILD_FUR: u32 = 10;

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

/// any crafting in parallel
pub fn smelting_parallel_1to1<R: FurnaceRecipe, const N: u32>(
    furs: &mut [Furnace<R>],
    res: Bundle<<R::Inputs as ResTuple1>::ResType, { N * R::InputBundle::RES_AMOUNT }>,
    tick: &mut Tick,
    show_time: bool,
) -> Bundle<<R::Outputs as ResTuple1>::ResType, N>
where
    R::Inputs: ResTuple1,
    R::Outputs: ResTuple1,
    R::InputBundle: ResBundleTuple1,
    R::OutputBundle: ResBundleTuple1,
{
    let start_tick = tick.cur();

    let in_factor = R::InputBundle::RES_AMOUNT;
    let out_factor = R::OutputBundle::RES_AMOUNT;

    let arangement = calculate_requirement::<N>(furs.len());
    let mut material = res.to_resource();

    for (fur, &res_num) in furs.iter_mut().zip(arangement.iter()) {
        let material = material.split_off(res_num * in_factor).unwrap();
        assign_furnance(fur, tick, material);
    }

    while !tick.advance_until(
        |tick| {
            furs.iter_mut()
                .zip(arangement.iter())
                .all(|(fur, &res_num)| {
                    fur.outputs(tick).access_res().amount() >= res_num * out_factor
                })
        },
        100,
    ) {}

    let mut res_total = Resource::new_empty();
    for (fur, &res_num) in furs.iter_mut().zip(arangement.iter()) {
        let Ok(res) = fur
            .outputs(tick)
            .access_res()
            .split_off(res_num * out_factor)
        else {
            continue;
        };
        res_total.add(res);
    }
    if show_time {
        println!(
            "Smelting {} cost {} ticks",
            res_total,
            tick.cur() - start_tick
        );
    }
    res_total.bundle().unwrap()
}

pub fn assemble_parallel_2to1<R: AssemblerRecipe, const N: u32>(
    assems: &mut [Assembler<R>],
    res1: Bundle<<R::Inputs as ResTuple2>::ResType1, { N * R::InputBundle::RES_AMOUNT1 }>,
    res2: Bundle<<R::Inputs as ResTuple2>::ResType2, { N * R::InputBundle::RES_AMOUNT2 }>,
    tick: &mut Tick,
) -> Bundle<<R::Outputs as ResTuple1>::ResType, N>
where
    R::Inputs: ResTuple2,
    R::Outputs: ResTuple1,
    R::InputBundle: ResBundleTuple2,
    R::OutputBundle: ResBundleTuple1,
{
    let in1_factor = R::InputBundle::RES_AMOUNT1;
    let in2_factor = R::InputBundle::RES_AMOUNT2;
    let out_factor = R::OutputBundle::RES_AMOUNT;

    let base_arangement = calculate_requirement::<N>(assems.len());
    let mut material1 = res1.to_resource();
    let mut material2 = res2.to_resource();

    for (assem, &res_num) in assems.iter_mut().zip(base_arangement.iter()) {
        let material1 = material1.split_off(res_num * in1_factor).unwrap();
        let material2 = material2.split_off(res_num * in2_factor).unwrap();
        assem.inputs(tick).access_res1().add(material1);
        assem.inputs(tick).access_res2().add(material2);
    }

    while !tick.advance_until(
        |tick| {
            assems
                .iter_mut()
                .zip(base_arangement.iter())
                .all(|(assem, &res_num)| {
                    assem.outputs(tick).access_res().amount() >= res_num * out_factor
                })
        },
        100,
    ) {}

    let mut res_total = Resource::new_empty();
    for (assem, &res_num) in assems.iter_mut().zip(base_arangement.iter()) {
        let Ok(res) = assem
            .outputs(tick)
            .access_res()
            .split_off(res_num * out_factor)
        else {
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
        let res = mine_resource(iron_territory, tick);
        let iron = smelting_parallel_1to1(&mut furs, res, tick, false);
        furs.push(Furnace::build(tick, IronSmelting, iron));
    }

    let res = mine_resource(iron_territory, tick);
    let iron = smelting_parallel_1to1(&mut furs, res, tick, false);

    let mut furs = change_recipe(furs, CopperSmelting);
    let res = mine_resource(copper_territory, tick);
    let copper = smelting_parallel_1to1(&mut furs, res, tick, false);

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
        .map(|fur| fur.change_recipe(new_recipe))
        .flatten()
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
    fur.inputs(&tick).access_res().add(res);
}

fn calculate_requirement<const AMOUNT: u32>(machine_num: usize) -> Vec<u32> {
    let mut initial = vec![(AMOUNT as usize / machine_num) as u32; machine_num];
    initial
        .iter_mut()
        .take(AMOUNT as usize % machine_num)
        .for_each(|i| *i += 1);
    initial
}
