mod shadow;
use shadow::bigworld::BigWorld;
use shadow::smallworld::SmallWorld;
use std::time;
fn main() {
    let mut smallworlds = Vec::new();
    smallworlds.push(SmallWorld::new(time::Duration::from_secs(2)));
    smallworlds.push(SmallWorld::new(time::Duration::from_secs(3)));
    let mut world = BigWorld::new(time::Duration::from_secs_f64(10.0), Some(smallworlds));
    world.run();
}
