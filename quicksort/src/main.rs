extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    println!("{:?}", recursive_quicksort(randomly_filled_vec(100)));
}

fn recursive_quicksort(numbers: Vec<i32>) -> Vec<i32> {
    if numbers.len() < 2 {
        return numbers;
    }

    let pivot: i32 = numbers[numbers.len() >> 1];
    let mut sorted: Vec<i32> = vec![];

    let mut smaller_than_pivot: Vec<i32> = vec![];
    let mut equal_to_pivot: Vec<i32> = vec![];
    let mut larger_than_pivot: Vec<i32> = vec![];

    for (_, &number) in numbers.iter().enumerate() {
        if number < pivot {
            smaller_than_pivot.push(number);
        }

        if number == pivot {
            equal_to_pivot.push(number);
        }

        if number > pivot {
            larger_than_pivot.push(number);
        }
    }

    sorted.append(&mut recursive_quicksort(smaller_than_pivot));
    sorted.append(&mut recursive_quicksort(equal_to_pivot));
    sorted.append(&mut recursive_quicksort(larger_than_pivot));
    sorted
}

fn randomly_filled_vec(length: i32) -> Vec<i32> {
    let mut numbers: Vec<i32> = (0..=length).collect();
    let mut rng = thread_rng();
    numbers.shuffle(&mut rng);
    numbers
}
