use rustorio::buildings::Assembler;
use rustorio::recipes::AssemblerRecipe;
use rustorio::{self, Bundle, Resource, Tick};

use crate::{ResBundleTuple1, ResBundleTuple2, ResTuple1, ResTuple2, calculate_requirement};

pub fn assemble_parallel_1to1<R: AssemblerRecipe, const N: u32>(
    assems: &mut [Assembler<R>],
    res: Bundle<<R::Inputs as ResTuple1>::ResType, { N * R::InputBundle::RES_AMOUNT }>,
    tick: &mut Tick,
) -> Bundle<<R::Outputs as ResTuple1>::ResType, { N * R::OutputBundle::RES_AMOUNT }>
where
    R::Inputs: ResTuple1,
    R::Outputs: ResTuple1,
    R::InputBundle: ResBundleTuple1,
    R::OutputBundle: ResBundleTuple1,
{
    let in_factor = R::InputBundle::RES_AMOUNT;
    let out_factor = R::OutputBundle::RES_AMOUNT;

    let base_arangement = calculate_requirement::<N>(assems.len());
    let mut material = res.to_resource();

    for (assem, &res_num) in assems.iter_mut().zip(base_arangement.iter()) {
        let material = material.split_off(res_num * in_factor).unwrap();
        assem.inputs(tick).access_res().add(material);
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

pub fn assemble_parallel_2to1<R: AssemblerRecipe, const N: u32>(
    assems: &mut [Assembler<R>],
    res1: Bundle<<R::Inputs as ResTuple2>::ResType1, { N * R::InputBundle::RES_AMOUNT1 }>,
    res2: Bundle<<R::Inputs as ResTuple2>::ResType2, { N * R::InputBundle::RES_AMOUNT2 }>,
    tick: &mut Tick,
) -> Bundle<<R::Outputs as ResTuple1>::ResType, { N * R::OutputBundle::RES_AMOUNT }>
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
