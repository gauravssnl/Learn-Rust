use std::cmp::Ordering;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    // Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    // Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches("s")
    }
}

// Enums with Data

// A timestamp that has been deliberately rounded off, so our program
// says "6 months ago" instead of "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32)
}
struct Point3d(f32, f32, f32);

enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cubouid { corner1: Point3d, corner2: Point3d }
}


enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
}

fn main() {
    println!("{}", mem::size_of::<Ordering>());
    println!("{}", mem::size_of_val(&Ordering::Less));

    println!("{}", "abc123".trim_end_matches(char::is_numeric));
    println!("{}", "abc123".trim_end_matches(|c| c == '1' || c == '2' || c == '3'));

    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
}
