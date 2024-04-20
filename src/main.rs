use std::fmt::Display;
use num::complex::Complex;
use std::ops::Add;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

// 引入第三方的哈希函数
use twox_hash::XxHash64;
struct Struct{
    e:i32
}
#[derive(Debug)]
 struct File {
   name: String,
   data: Vec<u8>,
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
    struct_test();
    test_enum();
    test_option();
    let r = test_match(Coin::AUD);

    println!("{}",r);
    test_if_let();
    let p = Point::new(1, 2);
    p.diff();
let p11 = P{x:1,y:2};

println!("{}",p11.x());


let xy1 = XY{x:1,y:2};
let xy2 = XY{x:3,y:4};
let xy3 = xy1+xy2;

println!("{}",xy3);

let person = Person;
person.fly();
Pilot::fly(&person);
test_vec();
test_hashmap();

}

fn test_hashmap(){
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let  teams_map:HashMap<_,_> =  teams_list.into_iter().collect();

    /**
    for k in &teams_list{
        teams_map.insert(&k.0, k.1);
    }
     */
    println!("{:?}",teams_map);


    let mut scores = HashMap::new();

    scores.insert(String::from("value"), 100);
    scores.insert(String::from("value2"), 200);

    for (k,v) in &scores{
        println!("{k}--{v}");
    }

    let b = scores.get(&String::from("value"));
    
}
fn test_vec(){

    let mut v = Vec::with_capacity(10);
    v.push(1);

    let mut k  =vec![1,2,3,4];
    let x = &k[2];

    k.sort();


    k.sort_by(|a,b| a.cmp(b));
}

struct Person;

trait Pilot {
    fn fly(&self);
}

trait  Wizard {
    fn fly(&self);
}

impl Pilot for Person{
    fn fly(&self){
        println!("Pilot----");
    }
}

impl  Wizard for Person {
    fn fly(&self){
        println!("Wizard----");
    }
}

impl  Person {
    fn fly(&self){
        println!("person----");
    } 
}

struct XY<T>{
    x:T,
    y:T
}

impl <T:Add<T,Output = T>> Add for XY<T> {

type Output = XY<T>;
    fn add(self, xy: XY<T>) -> Self::Output {
        XY{
            x:self.x+xy.x,
            y:self.y+xy.y
        }
    }
}

impl<T:Display> std::fmt::Display for XY<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "sunshine {{ x: {}, y: {} }}", self.x, self.y)
    }
}

pub trait Summary{
    fn summary(&self)->String;
}

pub struct Post{
    pub title:String,
    pub content:String,
}

impl Summary for Post{
    fn summary(&self)-> String {
        format!("文章{}", self.title)
    }
}

pub struct Weibo{
    pub title:String,
    pub content:String
}

impl Summary for Weibo{
    fn summary(&self)->String {
        format!("{}",self.title)
    }
}

fn test<T:Summary +Display>(p:&T)->String{
    String::new()
}
struct P<T>{
    x:T,
    y:T,
}
impl <T> P<T> {
    fn x(&self)->&T{
        &self.x
    }
}
struct Point{
    x:i32,
    y:i32
}
impl Point{

    fn new(x:i32,y:i32)->Point{
       Point { x: x, y: y}
    }
    fn diff(&self)->i32{
        return self.x-self.y;
    }
}
fn test_if_let(){

    let a = Some(100);
    println!("{:?}",a);
    if let Some(a) = a{
        println!("{:?}",a);
    }

    println!("{:?}",a);

}


enum Coin{
    JP,CN,AUD
}
fn test_match(coin:Coin)->i32{

    match coin{
        Coin::CN=>1,
        Coin::JP | Coin::AUD =>3,
        _=>2
    }

}

fn test_option(){

    let a = Some(5);
    let r = plus_one(a);
    println!("{:?}",r);
    
}

fn plus_one(data:Option<i32>)->Option<i32>{

    match data {
            Some(i)=>Some(i+1),
            None=>None
    }

}


#[derive(Debug)]
enum  PokerSuit{
    Hearts,
    Diamonds
}
fn test_enum(){
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    println!("{:?}",heart);
    println!("{:?}",diamond);
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
    tuple_test();

    let user = User {
            name:String::from("value"),
            age:32,
            male:true
    };
    //struct_test();
}



 fn struct_test() {
   let f1 = File {
     name: String::from("f1.txt"),
     data: Vec::new(),
   };

   let f1_name = &f1.name;
   let f1_length = &f1.data.len();

   println!("{:?}", f1);
   println!("{} is {} bytes long", f1_name, f1_length);
   println!("{:?}", f1);
    dbg!(&f1);
    dbg!(f1);
 }

struct User{
    name:String,
    age:i32,
    male:bool
}

fn tuple_test(){

    let tup = (1,2,3);
    let (x,y,z) = tup;
    
    print!("{}",x);


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
