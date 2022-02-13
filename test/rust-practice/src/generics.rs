struct Point<T> {
    x: T,
    y: T,
}

struct PointDiffer<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointDiffer<T, U> {
    fn mix<V, W>(self, other: PointDiffer<V, W>) -> PointDiffer<T, W> {
        PointDiffer {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![53, 423, 35, 199, 43];
    println!("{}", largest_i32(number_list));
    let char_list = vec!['a', 's', 'd', 'e'];
    println!("{}", largest_generics(char_list));

    // let p1 = Point { x: 1, y: 23 };
    // let p2 = Point { x: 1.3, y: 3.65 };
    let p3 = PointDiffer { x: 10.34, y: '3' };
    let p4 = PointDiffer { x: "test", y: 3};
    let p5 = p3.mix(p4);
    println!("{}, {}", p5.x, p5.y)
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for num in list {
        if num > largest {
            largest = num
        }
    }
    largest
}

fn largest_generics<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for num in list {
        if num > largest {
            largest = num
        }
    }
    largest
}