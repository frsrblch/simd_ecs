use simd_ecs::prelude::*;
use physics::*;
use simd_ecs::links::Insert;

fn main() {}

#[derive(Debug, Default, Clone)]
pub struct World {
    pub alloc: Allocators,
    pub state: State,
}

#[derive(Debug, Default, Clone)]
pub struct Allocators {
    pub system: FixedAllocator<System>,
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub system: System,
    pub body: Body,
}

#[derive(Debug, Default, Clone)]
pub struct System {
    pub name: Comp1<Self, String>,
    pub star_radius: Comp1<Self, Length>,
    pub star_temp: Comp1<Self, Temperature>,
}

impl System {
    pub fn create(&mut self, alloc: &mut FixedAllocator<Self>, row: SystemRow) -> Id<Self> {
        let id = alloc.create();
        self.insert(&id, row);
        id
    }

    fn insert(&mut self, id: &Id<Self>, row: SystemRow) {
        self.name.insert(id, row.name);
        self.star_radius.insert(id, row.star_radius);
        self.star_temp.insert(id, row.star_temp);
    }
}

#[derive(Debug, Clone)]
pub struct SystemRow {
    pub name: String,
    pub star_radius: Length,
    pub star_temp: Temperature,
}

#[derive(Debug, Default, Clone)]
pub struct Body {
    pub name: Comp1<Self, String>,
    pub mass: Comp1<Self, Mass>,
    pub radius: Comp1<Self, Length>,
    pub orbit_period: Comp1<Self, Time>,
    pub orbit_radius: Comp1<Self, Length>,
    pub orbit_offset: Comp1<Self, Angle>,

    pub system: Comp1<Self, Id<System>>,
}

impl Body {
    pub fn create(&mut self, alloc: &mut FixedAllocator<Self>, row: BodyRow, system: Id<System>) -> Id<Self> {
        let id = alloc.create();
        self.insert(&id, row);
        self.link(&id, system);
        id
    }

    fn insert(&mut self, id: &Id<Self>, row: BodyRow) {
        self.name.insert(id, row.name);
        self.mass.insert(id, row.mass);
        self.radius.insert(id, row.radius);
        self.orbit_period.insert(id, row.orbit_period);
        self.orbit_radius.insert(id, row.orbit_radius);
        self.orbit_offset.insert(id, row.orbit_offset);
    }

    fn link(&mut self, body: &Id<Body>, system: Id<System>) {
        self.system.insert(body, system);
    }
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit_period: Time,
    pub orbit_radius: Length,
    pub orbit_offset: Angle,
}