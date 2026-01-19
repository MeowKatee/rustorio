use rustorio::buildings::Furnace;
use rustorio::recipes::FurnaceRecipe;
use rustorio::{self, Bundle, Resource, Tick};

use crate::{ResBundleTuple1, ResTuple1, assign_furnance, calculate_requirement};

/// any crafting in parallel
pub fn smelting_parallel_1to1<R: FurnaceRecipe, const N: u32>(
    furs: &mut [Furnace<R>],
    res: Bundle<<R::Inputs as ResTuple1>::ResType, { N * R::InputBundle::RES_AMOUNT }>,
    tick: &mut Tick,
    show_time: bool,
) -> Bundle<<R::Outputs as ResTuple1>::ResType, { N * R::OutputBundle::RES_AMOUNT }>
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
