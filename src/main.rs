use fraction_calculator::{calculate_average, remove_first_and_last, insertion_sort, read_numeric_vec_from_file_vertical, read_numeric_vec_from_file_horizontal};

fn main() {
    let mut vec = read_numeric_vec_from_file_vertical::<i32>("num_vertical.txt").unwrap();
    println!("数组读取完毕");
    insertion_sort(&mut vec);
    println!("插入排序完毕");
    remove_first_and_last(&mut vec);
    println!("删除第一和最后一个");
    println!("最终评均分{:?}", calculate_average(&vec));
    println!("————————————————————————————————————");
    let mut vec = read_numeric_vec_from_file_horizontal::<i32>("num_horizontal.txt", ' ').unwrap();
    println!("数组读取完毕");
    insertion_sort(&mut vec);
    println!("插入排序完毕");
    remove_first_and_last(&mut vec);
    println!("删除第一和最后一个");
    println!("最终评均分{:?}", calculate_average(&vec));
}
