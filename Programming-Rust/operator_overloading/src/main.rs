mod lib;
use lib::Interval;
use lib::{Student, StudentList};

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

    let mut student_list = StudentList {
        list: vec![
            Student {
                id: 1,
                name: "GS SU".to_string(),
            },
            Student {
                id: 2,
                name: "Godi".to_string(),
            },
        ],
    };
    println!("{:?}", student_list);
    let first_student = &student_list[0];
    println!("{:?}", first_student);
    let mut first_student = &mut student_list[0];
    first_student.name = "Gaurav".to_string();
    println!("{:?}", student_list);
}
