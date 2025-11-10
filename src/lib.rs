use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub fn insertion_sort<T>(vec:&mut Vec<T>)
where T: PartialOrd + Copy{
    for i in 1..vec.len() {
        let mut j = i;
        let key = vec[i];
        while j > 0 && vec[j-1] > key {
            vec[j] = vec[j-1];
            j -= 1;
        }
        vec[j] = key;
    }
}

pub fn insertion_sort_swap<T: PartialOrd>(vec: &mut [T]) {
    for i in 1..vec.len() {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn remove_first_and_last<T>(vec: &mut Vec<T>) -> Option<(T, T)> {
    if vec.len() < 2 {
        return None;
    }
    let last = vec.pop().unwrap();
    let first = vec.remove(0);
    Some((first, last))
}

pub fn calculate_average<T>(vec: &[T]) -> f64
where
    T: Copy + Into<f64>,
{
    if vec.is_empty() {
        println!("这个向量太小");
        return 0.0;
    }

    let sum: f64 = vec.iter().map(|&x| x.into()).sum();
    sum / vec.len() as f64
}

pub fn read_numeric_vec_from_file_vertical<T>(filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    T::Err: std::error::Error + 'static,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        match trimmed.parse::<T>() {
            Ok(value) => result.push(value),
            Err(e) => return Err(Box::new(e)),
        }
    }

    Ok(result)
}

pub fn read_numeric_vec_from_file_horizontal<T>(
    filename: &str,
    delimiter: char
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    T::Err: std::error::Error + 'static,
{
    let content = std::fs::read_to_string(filename)?;

    let result: Result<Vec<T>, _> = content
        .split(delimiter)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().map_err(|e| Box::new(e) as Box<dyn std::error::Error>))
        .collect();

    result
}

#[test]
fn insertion_sort_test() {
    let mut vec = vec![1, 9, 2, 4, 3];
    insertion_sort(&mut vec);
    println!("{:?}", vec);
}

#[test]
fn insertion_swap_test(){
    let mut vec = vec![1, 9, 2, 4, 3];
    insertion_sort_swap(&mut vec);
    println!("{:?}", vec);
}

#[test]
fn remove_the_first_and_last_test() {
    let mut vec = vec![1, 2, 3, 4];
    remove_first_and_last(&mut vec);
    println!("{:?}", vec);
}

#[test]
fn calculate_average_score_test() {
    let vec = vec![3, 3, 9];
    println!("{:?}", calculate_average(&vec))
}

#[test]
fn read_vec_in_txt_test_vertical() {
    let vec = read_numeric_vec_from_file_vertical::<i32>("num_vertical.txt");
    println!("{:?}", vec);
}

#[test]
fn read_vec_in_txt_test_horizontal() {
    let vec = read_numeric_vec_from_file_horizontal::<i32>("num_horizontal.txt", ' ');
    println!("{:?}", vec);
}