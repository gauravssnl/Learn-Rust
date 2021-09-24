use std::ops::Range;

struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1..self.y
    }
}
struct Canvas {
    top_left: i32,
    right_bottom: i32,
    width: i32,
    height: i32,
}

impl Canvas {
    fn write_at(&mut self, x: i32, y: i32, c: char) {}
}
trait Visible {
    fn draw(&self, canvas: &mut Canvas);

    fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

fn main() {}
