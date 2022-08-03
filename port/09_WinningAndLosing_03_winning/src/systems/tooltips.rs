use crate::components;
use crate::prelude::*;

pub fn tooltips(
    positions: Query<(&PointC, &components::Name, Option<&Health>)>,
    (mouse_pos, camera): (Res<Point>, Res<Camera>),
) {
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    for (pos, name, health) in positions.iter() {
        if pos.0 == map_pos {
            let screen_pos = *mouse_pos * 4;
            let display = if let Some(health) = health {
                format!("{} : {} hp", &name.0, health.current)
            } else {
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        }
    }
    draw_batch.submit(10100).expect("Batch error");
}
