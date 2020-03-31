use simd_ecs::prelude::*;
use physics::*;

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
    pub colony: Colony,
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
    pub orbit_parent: Comp1<Self, Option<Id<Self>>>,
    pub orbit_period: Comp1<Self, Time>,
    pub orbit_radius: Comp1<Self, Length>,
    pub orbit_offset: Comp1<Self, Angle>,

    pub position: Comp2<Self, Length, Length>,
    pub velocity: Comp2<Self, Speed, Speed>,
    pub parent_position: Comp2<Self, Length, Length>,

    pub system: Comp1<Self, Id<System>>,
}

impl Body {
    pub fn create(&mut self, alloc: &mut FixedAllocator<Self>, row: BodyRow, system: Id<System>, parent: Option<Id<Self>>) -> Id<Self> {
        let id = alloc.create();
        self.insert(&id, row);
        self.link_system(&id, system);
        self.link_parent(&id, parent);
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

    fn link_system(&mut self, body: &Id<Body>, system: Id<System>) {
        self.system.insert(body, system);
    }

    fn link_parent(&mut self, body: &Id<Body>, parent: Option<Id<Body>>) {
        self.orbit_parent.insert(body, parent);
    }

    pub fn update_position(&mut self, time: Time) {
        self.calculate_relative_position(time);
        self.update_parent_position();
        self.calculate_absolute_position();
    }

    fn calculate_relative_position(&mut self, time: Time) {
        self.position.0.iter_mut()
            .zip(self.position.1.iter_mut())
            .zip(self.orbit_period.iter())
            .zip(self.orbit_radius.iter())
            .zip(self.orbit_offset.iter())
            .for_each(|((((x, y), period), radius), offset)| {
                let fraction = time / *period;
                let angle = Angle::in_degrees(360.0) * fraction + *offset;
                let cos = angle.cos();
                let sin = angle.sin();
                let x0 = Length::zero();
                let y0 = *radius;

                *x = x0 * cos - y0 * sin;
                *y = x0 * sin + y0 * cos;
            });
    }

    fn update_parent_position(&mut self) {
        let position = &self.position;
        let orbit_parent = &self.orbit_parent;
        self.parent_position.zip_to_comp1(
            orbit_parent,
            |x, y, parent| {
                *x = Length::zero();
                *y = Length::zero();
                let mut parent = *parent;
                while let Some((parent_x, parent_y)) = position.get(parent) {
                    *x += *parent_x;
                    *y += *parent_y;
                    parent = orbit_parent.get(parent).copied().flatten();
                }
            });
    }

    fn calculate_absolute_position(&mut self) {
        self.position.zip_each_to_comp2(&self.parent_position, |body, parent| *body += *parent);
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

#[derive(Debug, Default, Clone)]
pub struct Colony {
    pub name: Comp1<Self, String>,
    pub population: Comp1<Self, Population>,
    pub growth_rate: Comp1<Self, GrowthRate>,

    pub body: Comp1<Self, Id<Body>>,
}

impl Colony {
    pub fn create(&mut self, alloc: &mut DynamicAllocator<Self>, row: ColonyRow, body: Id<Body>) -> GenId<Self> {
        let id = alloc.create();
        self.insert(&id, row);
        self.link(&id, body);
        id
    }

    fn insert(&mut self, id: &GenId<Self>, row: ColonyRow) {
        self.name.insert(id, row.name);
        self.population.insert(id, row.population);
        self.growth_rate.insert(id, row.growth_rate);
    }

    fn link(&mut self, id: &GenId<Self>, body: Id<Body>) {
        self.body.insert(id, body);
    }
}

#[derive(Debug, Default, Clone)]
pub struct ColonyRow {
    pub name: String,
    pub population: Population,
    pub growth_rate: GrowthRate,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Population(pub f64);

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct GrowthRate(pub f64);

