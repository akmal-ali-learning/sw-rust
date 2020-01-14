// Chapter 1 : problem 7 
// Rotate Matrix: Given  an  image  represented  by an  NxN  matrix, where each  pixel  in  the  image  is  4  bytes, write a method to rotate the image by 90 degrees. Can you do this in  place? 

#[cfg(test)]
mod problem_7_test {
	use super::*;

    #[test]
    fn rotate_90_test() {
    	// let x = [
	    // 	[ 1     ,2     ,3     ,4],
	    // 	[ 5     ,6     ,7     ,8],
	    // 	[ 9     ,10    ,11    ,12],
	    // 	[ 13    ,14    ,15	  ,16]
	    // 	];


	    // let mut x = [1,2,3,
	    // 			 4,5,6,
	    // 			 7,8,9];
	    // println!("{:?}",x );

	    // let expected_x = [7,4,1,
		   //  			  8,5,2,
		   //  			  9,6,3];

       let mut x = [ 1, 2, 3,
       				 4, 5, 6,
       				 7, 8, 9
       				];


       let y = [7,4,1,
				8,5,2,
				9,6,3];



		// println!("{:?}",x );

    	rotate90(&mut x);

    	// println!("{:?}",x );
    	// println!("{:?}",y );
        // assert_eq!(x, y);

        let mut x = [ 	1, 2, 3, 4, 5,
        				6, 7 ,8, 9, 10,
        				11,12,13,14,15,
        				16,17,18,19,20,
        				21,22,23,24,25
        				];

        let y 	= 	[   21,16,11, 6, 1,
        				22,17,12, 7, 2,
        				23,18,13, 8, 3,
        				24,19,14, 9, 4,
        				25,20,15,10, 5
        				];

		rotate90(&mut x);
		assert_eq!(x, y);


    }
}

#[derive(Debug)]
struct Point {
	x: usize,
	y: usize,
}

fn rotate90( input_array : &mut [u32])
{
	assert!(is_square(input_array.len()));

	let n = (input_array.len() as f64).sqrt() as usize;

	// To rotate an N by N image by 90 degrees.
	for i_row in  0 .. n/2
	{
		for i_col in 0 .. n/2
		{
			let pixel_a = Point{ x : i_row, 			y : i_col};
			let pixel_b = Point{ x : n - 1- i_row, 		y : i_col};
			let pixel_c = Point{ x : n - 1- i_row,		y : n - 1 -i_col};
			let pixel_d = Point{ x : i_row, 			y : n - 1 -i_col};

			// println!("{:?} {:?} {:?} {:?}", pixel_a, pixel_b, pixel_c, pixel_d );
			let temp = input_array[pixel_a.x*n + pixel_a.y];
			input_array[pixel_a.x*n + pixel_a.y ]= input_array[pixel_b.x*n + pixel_b.y];
			input_array[pixel_b.x*n + pixel_b.y ]= input_array[pixel_c.x*n + pixel_c.y];
			input_array[pixel_c.x*n + pixel_c.y ]= input_array[pixel_d.x*n + pixel_d.y];
			input_array[pixel_d.x*n + pixel_d.y ]= temp;
		}
	}

	// for i_row in  0 .. n/2
	// {
	// 	for i_col in n/2 .. (n+1)/2
	// 	{
	// 		let pixel_a = Point{ x : i_row, 			y : i_col};
	// 		let pixel_c = Point{ x : n - 1 -i_row ,		y : n - 1- i_col};
	// 		let pixel_b = Point{ x : i_col, 			y : i_row};
	// 		let pixel_d = Point{ x : i_col, 			y : n - 1- i_row};

	// 		println!("{:?} {:?} {:?} {:?}", pixel_a, pixel_b, pixel_c, pixel_d );
	// 		let temp = input_array[pixel_a.x*n + pixel_a.y];
	// 		input_array[pixel_a.x*n + pixel_a.y ]= input_array[pixel_b.x*n + pixel_b.y];
	// 		input_array[pixel_b.x*n + pixel_b.y ]= input_array[pixel_c.x*n + pixel_c.y];
	// 		input_array[pixel_c.x*n + pixel_c.y ]= input_array[pixel_d.x*n + pixel_d.y];
	// 		input_array[pixel_d.x*n + pixel_d.y ]= temp;
	// 	}
	// }

}

fn is_square(in_x : usize) -> bool
{
	let a = (in_x as f64).sqrt() as usize;

	in_x == a * a
}


// @todo learn about Rust Arrays
// An array is a collection of object of the same type T
// stored in contiguous memory.contiguous

//Created at compile time.