use rustorio::{Resource, ResourceType};

pub trait ResTuple1 {
    type ResType: ResourceType;
    fn access_res(&mut self) -> &mut Resource<Self::ResType>;
}

impl<R> ResTuple1 for (Resource<R>,)
where
    R: ResourceType,
{
    type ResType = R;
    fn access_res(&mut self) -> &mut Resource<Self::ResType> {
        &mut self.0
    }
}

pub trait ResTuple2 {
    type ResType1: ResourceType;
    type ResType2: ResourceType;

    fn access_res1(&mut self) -> &mut Resource<Self::ResType1>;
    fn access_res2(&mut self) -> &mut Resource<Self::ResType2>;
}

impl<R1, R2> ResTuple2 for (Resource<R1>, Resource<R2>)
where
    R1: ResourceType,
    R2: ResourceType,
{
    type ResType1 = R1;
    type ResType2 = R2;

    fn access_res1(&mut self) -> &mut Resource<Self::ResType1> {
        &mut self.0
    }
    fn access_res2(&mut self) -> &mut Resource<Self::ResType2> {
        &mut self.1
    }
}

pub trait ResTuple3 {
    type ResType1: ResourceType;
    type ResType2: ResourceType;
    type ResType3: ResourceType;

    fn access_res1(&mut self) -> &mut Resource<Self::ResType1>;
    fn access_res2(&mut self) -> &mut Resource<Self::ResType2>;
    fn access_res3(&mut self) -> &mut Resource<Self::ResType3>;
}

impl<R1, R2, R3> ResTuple3 for (Resource<R1>, Resource<R2>, Resource<R3>)
where
    R1: ResourceType,
    R2: ResourceType,
    R3: ResourceType,
{
    type ResType1 = R1;
    type ResType2 = R2;
    type ResType3 = R3;

    fn access_res1(&mut self) -> &mut Resource<Self::ResType1> {
        &mut self.0
    }
    fn access_res2(&mut self) -> &mut Resource<Self::ResType2> {
        &mut self.1
    }
    fn access_res3(&mut self) -> &mut Resource<Self::ResType3> {
        &mut self.2
    }
}

pub trait ResTuple4 {
    type ResType1: ResourceType;
    type ResType2: ResourceType;
    type ResType3: ResourceType;
    type ResType4: ResourceType;

    fn access_res1(&mut self) -> &mut Resource<Self::ResType1>;
    fn access_res2(&mut self) -> &mut Resource<Self::ResType2>;
    fn access_res3(&mut self) -> &mut Resource<Self::ResType3>;
    fn access_res4(&mut self) -> &mut Resource<Self::ResType4>;
}

impl<R1, R2, R3, R4> ResTuple4 for (Resource<R1>, Resource<R2>, Resource<R3>, Resource<R4>)
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

    fn access_res1(&mut self) -> &mut Resource<Self::ResType1> {
        &mut self.0
    }
    fn access_res2(&mut self) -> &mut Resource<Self::ResType2> {
        &mut self.1
    }
    fn access_res3(&mut self) -> &mut Resource<Self::ResType3> {
        &mut self.2
    }
    fn access_res4(&mut self) -> &mut Resource<Self::ResType4> {
        &mut self.3
    }
}
