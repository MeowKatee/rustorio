use rustorio::{Bundle, ResourceType};

pub trait ResBundleTuple1 {
    type ResType: ResourceType;
    const RES_AMOUNT: u32;
}

pub trait ResBundleTuple2 {
    type ResType1: ResourceType;
    type ResType2: ResourceType;
    const RES_AMOUNT1: u32;
    const RES_AMOUNT2: u32;
}

pub trait ResBundleTuple3 {
    type ResType1: ResourceType;
    type ResType2: ResourceType;
    type ResType3: ResourceType;
    const RES_AMOUNT1: u32;
    const RES_AMOUNT2: u32;
    const RES_AMOUNT3: u32;
}

pub trait ResBundleTuple4 {
    type ResType1: ResourceType;
    type ResType2: ResourceType;
    type ResType3: ResourceType;
    type ResType4: ResourceType;
    const RES_AMOUNT1: u32;
    const RES_AMOUNT2: u32;
    const RES_AMOUNT3: u32;
    const RES_AMOUNT4: u32;
}

impl<R, const N: u32> ResBundleTuple1 for (Bundle<R, N>,)
where
    R: ResourceType,
{
    type ResType = R;
    const RES_AMOUNT: u32 = N;
}

impl<R1, const N1: u32, R2, const N2: u32> ResBundleTuple2 for (Bundle<R1, N1>, Bundle<R2, N2>)
where
    R1: ResourceType,
    R2: ResourceType,
{
    type ResType1 = R1;
    type ResType2 = R2;
    const RES_AMOUNT1: u32 = N1;
    const RES_AMOUNT2: u32 = N2;
}

impl<R1, const N1: u32, R2, const N2: u32, R3, const N3: u32> ResBundleTuple3
    for (Bundle<R1, N1>, Bundle<R2, N2>, Bundle<R3, N3>)
where
    R1: ResourceType,
    R2: ResourceType,
    R3: ResourceType,
{
    type ResType1 = R1;
    type ResType2 = R2;
    type ResType3 = R3;
    const RES_AMOUNT1: u32 = N1;
    const RES_AMOUNT2: u32 = N2;
    const RES_AMOUNT3: u32 = N3;
}

impl<R1, const N1: u32, R2, const N2: u32, R3, const N3: u32, R4, const N4: u32> ResBundleTuple4
    for (
        Bundle<R1, N1>,
        Bundle<R2, N2>,
        Bundle<R3, N3>,
        Bundle<R4, N4>,
    )
where
    R1: ResourceType,
    R2: ResourceType,
    R3: ResourceType,
    R4: ResourceType,
{
    type ResType1 = R1;
    type ResType2 = R2;
    type ResType3 = R3;
    type ResType4 = R4;
    const RES_AMOUNT1: u32 = N1;
    const RES_AMOUNT2: u32 = N2;
    const RES_AMOUNT3: u32 = N3;
    const RES_AMOUNT4: u32 = N4;
}
