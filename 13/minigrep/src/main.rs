use std::thread;
use std::time::Duration;


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating {}...", intensity);
    thread::sleep(Duration::from_secs(2));
    intensity
}


fn generate_workout (intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}


// 定义一个闭包
// 以一对竖线（|）开始，在竖线中指定闭包的参数，有多个时使用逗号隔开，|param1, param2|
// 闭包末尾需要使用分号结尾
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};


fn main() {
    // let answer = simulated_expensive_calculation(10);
    // println!("answer: {}", answer);

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7; 
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
