pub fn variables(){

    println!("Hello, world!");

    let mut x  = 5;
    let mut x="six";
    
    println!("{}",x);
    let tup=("hello world",23);
    let (channel, sub_count) = tup;
    let error_codes =[5,15,5,6];
    let names =["merhaba","hello"];
    let not_found= error_codes[1];
    let mut x = error_codes[3];
    let numbers: [i32; 8]=[0;8];
    
     for i in 0..8{
        println!("numbers[{}]={}",i,numbers[i]);
     }

}