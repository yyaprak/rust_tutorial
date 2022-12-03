pub fn func(){
    println!("Hello, world!");
    let sum=my_function(5,45);
    println!("sum of x+y = {}",sum);
}

fn my_function(x:i32, y:i32)->i32 {

    println!("another function x:{},y:{}",x,y);
    x+y
}