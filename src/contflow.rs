pub fn contflow(){

    let number = 25;
    if number < 10 {
        println!("first condition was true!!!!");
    }else if number < 22 {
        println!("second condition was true");
    }else {
        println!("condition was false");
    }
    
    let condition =false;
    let number = if condition {5} else{6};
    println!("number {}",number);

}