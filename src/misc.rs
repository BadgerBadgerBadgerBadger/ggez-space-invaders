use ggez::graphics::Rect;

// given two rectangles on a 2D plane, determines if the rectangles overlap
pub fn overlaps(box1: &Rect, box2: &Rect) -> bool {
    let x_min_1 = box1.x;
    let x_max_1 = box1.x + box1.w;
    let x_min_2 = box2.x;
    let x_max_2 = box2.x + box2.w;

    let y_min_1 = box1.y;
    let y_max_1 = box1.y + box1.h;
    let y_min_2 = box2.y;
    let y_max_2 = box2.y + box2.h;

    return overlaps_1d(x_max_1, x_min_1, x_max_2, x_min_2)
        && overlaps_1d(y_max_1, y_min_1, y_max_2, y_min_2);
}

// given the start and end of two segments on a 1D line,
// determines whether the segments overlap
fn overlaps_1d(x_max_1: f32, x_min_1: f32, x_max_2: f32, x_min_2: f32) -> bool {
    return (x_max_1 > x_min_2 && x_min_1 < x_max_2) || (x_max_2 > x_min_1 && x_min_2 < x_max_1);
}
