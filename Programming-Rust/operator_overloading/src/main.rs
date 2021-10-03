mod lib;
use lib::Interval;

fn main() {
    assert!(
        Interval {
            lower: 10,
            upper: 20
        } < Interval {
            lower: 20,
            upper: 40
        }
    );

    let mut intervals = vec![
        Interval {
            lower: 10,
            upper: 20,
        },
        Interval {
            lower: 20,
            upper: 40,
        },
        Interval {
            lower: 5,
            upper: 10,
        },
    ];

    intervals.sort_by_key(|i| i.upper);
    println!("intervals: {:?}", intervals);
    // Ord is used in sorting
    // Reverse wrapper takes adavtange of Ord that simply inverts any ordering.
    intervals.sort_by_key(|i| std::cmp::Reverse(i.upper));
    println!("intervals: {:?}", intervals);
}
