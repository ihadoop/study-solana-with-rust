use num::complex::Complex;
struct Struct{
    e:i32
}
fn main() {

    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2,8,99,10, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

   let k  = (-42_f32).sqrt();
   if !k.is_nan(){
   print!("k is:{}",k);
}

let f = 1.03323f32;

print!("{:.2}",f);


for i in 1..=5{
    println!("{}",i);
}


let a = Complex{re:2.1,im:-1.2};
let b = Complex::new(11.1, 22.2);
let result = a+b;
println!("{}+{}i",result.re,result.im);

println!("{}",std::mem::size_of_val(&f));



let kk = {
    1+2
};


println!("{}",kk);
plus_or_minus(1_i32);


let mut s  = String::from("ddd");
s.push_str(",dddd");
takes_ownership(s.clone());

println!("{}",s);

 let test_x = 5;
 let test_y = &test_x;
 assert_eq!(5,test_x);
 assert_eq!(5,*test_y);

    let str = String::from("Hello");
    let length = calculate_length(&str);

    println!("str :{},length:{}",str,length);
    
    let new_str = "ddddd";

    string_test();


}


fn string_test(){

let mut str = String::from("hello,");
str.push_str("world");
println!("{}",str);

str.insert_str(4, "abc");
println!("{}",str);

dbg!(str.pop());

let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    

    let s1 = String::from("hello");
    let s2  =String::from("world");
    let s3 = s1 + &s2;
    dbg!(s2);
    let s4 = format!("{}",s3);
    dbg!(s4);
    //dbg!(s4);

}
fn greet(name: String) {
    println!("Hello, {}!", name);
  }

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} 

fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }

    x + 5
}

fn demo() {
    let datas  ="\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

   let records = datas.lines();
   for (i,record) in records.enumerate(){
       if i==0 || record.trim().len()==0{
        continue;
       }
    
    let fields:Vec<_> =record.split(",").map(|field| field.trim()).collect();

    if cfg!(debug_assertions){
       eprintln!("debug: {:?} -> {:?}",
       record, fields);
    }
    let name = fields[0];
    if let Ok(length)= fields[1].parse::<f32>(){
        println!("{},{}cm",name,length);
        
    }
   }
}



fn greet_world(){
    let chinese = "你好";
    let en = "Hello World";
    let southern_germany = "Grüß Gott!";
    let arr  =[chinese,en,southern_germany];

    for hello in arr.iter(){
        println!("{}",hello);
    }

    let _x=5;




}
