use std::num;

pub fn loops(){
    let mut counter = 0;
    
    let  result=loop{
        println!("loops..again{}",counter);
        counter+=1;
        if counter== 10{
            break counter;
        }
        
    };
    println!("the result is {}",result);
   
    //example of while
    let mut number=3;
    while number!=0 {
       
        println!("{}",number);
        number-=1;

    }
    println!("LIFTOFF");

    //for iteration
    let a =[10,20,30,40,50];

    for element in a.iter(){
        println!("the value is {}",element);
    }
    //or
    for i in(0..5){
        println!("the value is {}",a[i]);
    }

}