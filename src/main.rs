use rand::Rng;
use sorting_algorithms::ShellSort;
use time::Instant;

fn main() {

    let mut num_list: Vec<isize> = Vec::new();

    for _ in 0..131072 {
        num_list.push(rand::thread_rng().gen());
    }

    let num_list: Vec<isize> = num_list.to_owned();

    let  (list_1024, _) = num_list.split_at(1024);
    let (list_4096, _) = num_list.split_at(4096);
    let (list_16384, _) = num_list.split_at(16384);
    let (list_131072, _) = num_list.split_at(131072);

    println!("Running for 1023 elements");
    selection_sort_profiling(list_1024.to_vec().as_ref());
    println!("Running for 4096 elements");
    selection_sort_profiling(list_4096.to_vec().as_ref());
    println!("Running for 16384 elements");
    selection_sort_profiling( list_16384.to_vec().as_ref());
    println!("Running for 131072 elements");
    selection_sort_profiling( list_131072.to_vec().as_ref());


    
}

fn selection_sort_profiling(num_list: &Vec<isize>) {
    for k in 0..10 {
        let start_time = time::Instant::now();
        ShellSort::sort(&num_list.to_owned());

        let end_time = time::Instant::now();
        let delta = end_time - start_time;

        println!("Run {}: {}", k, delta.as_seconds_f64());
    }
}