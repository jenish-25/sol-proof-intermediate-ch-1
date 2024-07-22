use std::io;
pub fn adder(num1:f64,num2:f64)->f64{
    let res=num1+num2;
    res
}
pub fn sub(num1:f64,num2:f64)->f64{
    let res=num1-num2;
    res
}
pub fn multi(num1:f64,num2:f64)->f64{
    let res=num1*num2;
    res
}
pub fn divisible(num1:f64,num2:f64)->f64{
    let mut res=0.0;
    if num2 != 0.0{
        res=num1/num2;
    }else{
        println!("enter a valid input");
    }
    return res;
}

fn main(){
    println!("calculator");

    println!("enter a operator (+ , - , * , /)");
    let mut operator=String::new();

    io::stdin().read_line(&mut operator).expect("failed to read line");
    let operator=operator.trim();
    println!("enter a first num");

    let mut num1=String::new();
    io::stdin().read_line(&mut num1).expect("failed to read line");

   let num1:f64=num1.trim().parse().expect("please type a number");

    println!("enter a second num");
    let mut num2=String::new();
    io::stdin().read_line(&mut num2).expect("failed to read line");

   let num2:f64=num2.trim().parse().expect("please type a number");

    let result=match operator{
        "+" => adder(num1,num2),
        "-" => sub(num1,num2),
        "*" => multi(num1,num2),
        "/" => divisible(num1,num2),
    _ => {
        println!("Error: Invalid operator");
        return;
    }
};
println!("reuslt of {} {} {} is {} ",num1,operator,num2,result);
}
