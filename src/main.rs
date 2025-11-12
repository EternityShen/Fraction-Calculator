use fraction_calculator::{calculate_average, remove_first_and_last, insertion_sort, read_numeric_vec_from_file_vertical, read_numeric_vec_from_file_horizontal, add_all_numbers_sum};

fn main() {
    let mut vec = read_numeric_vec_from_file_vertical::<i32>("num_vertical.txt").unwrap();
    println!("数组读取完毕");
    println!("{:?}", vec);
    insertion_sort(&mut vec);
    println!("插入排序完毕");
    println!("{:?}", vec);
    remove_first_and_last(&mut vec);
    println!("删除第一和最后一个");
    println!("{:?}", vec);
    println!("最终评均分{:?}", calculate_average(&vec));
    println!("最终总分{:?}", add_all_numbers_sum(&mut vec));
    println!("————————————————————————————————————");
    let mut vec = read_numeric_vec_from_file_horizontal::<f32>("num_horizontal.txt", "	").unwrap();
    println!("数组读取完毕");
    println!("{:?}", vec);
    insertion_sort(&mut vec);
    println!("插入排序完毕");
    println!("{:?}", vec);
    remove_first_and_last(&mut vec);
    println!("删除第一和最后一个");
    println!("{:?}", vec);
    println!("最终评均分{:?}", calculate_average(&vec));
    println!("最终总分{:?}", add_all_numbers_sum(&mut vec));
}
