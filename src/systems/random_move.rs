use crate::prelude::*;

#[system]
#[read_component(MovingRandomly)]
#[write_component(Point)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <&mut Point>::query()
        .filter(component::<MovingRandomly>());
    movers
        .iter_mut(ecs)
        .for_each(|pos| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            if map.can_enter_tile(destination) {
                *pos = destination;
            }
        }
    );
}