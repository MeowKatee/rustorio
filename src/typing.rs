use rustorio::{Resource, ResourceType};

pub trait ResTuple1 {
    type RESOURCE: ResourceType;
    fn access_res(&mut self) -> &mut Resource<Self::RESOURCE>;
}

impl<R> ResTuple1 for (Resource<R>,)
where
    R: ResourceType,
{
    type RESOURCE = R;
    fn access_res(&mut self) -> &mut Resource<Self::RESOURCE> {
        &mut self.0
    }
}

pub trait ResTuple2 {
    type RESOURCE1: ResourceType;
    type RESOURCE2: ResourceType;

    fn access_res1(&mut self) -> &mut Resource<Self::RESOURCE1>;
    fn access_res2(&mut self) -> &mut Resource<Self::RESOURCE2>;
}

impl<R1, R2> ResTuple2 for (Resource<R1>, Resource<R2>)
where
    R1: ResourceType,
    R2: ResourceType,
{
    type RESOURCE1 = R1;
    type RESOURCE2 = R2;

    fn access_res1(&mut self) -> &mut Resource<Self::RESOURCE1> {
        &mut self.0
    }
    fn access_res2(&mut self) -> &mut Resource<Self::RESOURCE2> {
        &mut self.1
    }
}

pub trait ResTuple3 {
    type RESOURCE1: ResourceType;
    type RESOURCE2: ResourceType;
    type RESOURCE3: ResourceType;

    fn access_res1(&mut self) -> &mut Resource<Self::RESOURCE1>;
    fn access_res2(&mut self) -> &mut Resource<Self::RESOURCE2>;
    fn access_res3(&mut self) -> &mut Resource<Self::RESOURCE3>;
}

impl<R1, R2, R3> ResTuple3 for (Resource<R1>, Resource<R2>, Resource<R3>)
where
    R1: ResourceType,
    R2: ResourceType,
    R3: ResourceType,
{
    type RESOURCE1 = R1;
    type RESOURCE2 = R2;
    type RESOURCE3 = R3;

    fn access_res1(&mut self) -> &mut Resource<Self::RESOURCE1> {
        &mut self.0
    }
    fn access_res2(&mut self) -> &mut Resource<Self::RESOURCE2> {
        &mut self.1
    }
    fn access_res3(&mut self) -> &mut Resource<Self::RESOURCE3> {
        &mut self.2
    }
}

pub trait ResTuple4 {
    type RESOURCE1: ResourceType;
    type RESOURCE2: ResourceType;
    type RESOURCE3: ResourceType;
    type RESOURCE4: ResourceType;

    fn access_res1(&mut self) -> &mut Resource<Self::RESOURCE1>;
    fn access_res2(&mut self) -> &mut Resource<Self::RESOURCE2>;
    fn access_res3(&mut self) -> &mut Resource<Self::RESOURCE3>;
    fn access_res4(&mut self) -> &mut Resource<Self::RESOURCE4>;
}

impl<R1, R2, R3, R4> ResTuple4 for (Resource<R1>, Resource<R2>, Resource<R3>, Resource<R4>)
where
    R1: ResourceType,
    R2: ResourceType,
    R3: ResourceType,
    R4: ResourceType,
{
    type RESOURCE1 = R1;
    type RESOURCE2 = R2;
    type RESOURCE3 = R3;
    type RESOURCE4 = R4;

    fn access_res1(&mut self) -> &mut Resource<Self::RESOURCE1> {
        &mut self.0
    }
    fn access_res2(&mut self) -> &mut Resource<Self::RESOURCE2> {
        &mut self.1
    }
    fn access_res3(&mut self) -> &mut Resource<Self::RESOURCE3> {
        &mut self.2
    }
    fn access_res4(&mut self) -> &mut Resource<Self::RESOURCE4> {
        &mut self.3
    }
}
