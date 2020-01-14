fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);


    let point = Point { x : 5	, y : 4    };
    // println!("Distance = {} ", point.distance_from_origin() );
    println!("p.x = {} p.y = {} ", point.x(), point.y() );

    let point = Point { x : 5.0 , y : 4.0  };
    println!("Distance = {} ", point.distance_from_origin() );

    println!("p.x = {} p.y = {} ", point.x(), point.y() );
    // let point = Point { x : 5 	, y : 4.0  };
    // Use a function.
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_number(&number_list);

    // println!("The largest_number number is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest_number(&number_list);

    // println!("The largest_number number is {}", result);

    //
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}

struct Point<T,U> {
	x : T,
	y : U,
}

impl<T,U> Point<T,U> {
	fn x(&self) -> &T {
		&self.x
	}
	fn y(&self) -> &U {
		&self.y
	}

	fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
		Point {
			x : self.x,
			y: other.y,
		}
	}

}


impl Point<f32,f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



// fn largest_number<T>(list: &[T]) -> T {
//     let mut largest_val = list[0];

//     for &item in list.iter() {
//         if item > largest_val {
//             largest_val = item;
//         }
//     }

//     largest_val
// }
