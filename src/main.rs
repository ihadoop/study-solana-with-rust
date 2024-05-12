use mysql::consts;
use num::complex::Complex;
use core::fmt;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::hash::BuildHasherDefault;
use std::io::{self, ErrorKind, Read};
use std::net::Ipv4Addr;
use std::ops::Add;
use std::sync::{Arc, Barrier,Mutex, Condvar,Once};
use std::thread;

// 引入第三方的哈希函数
use twox_hash::XxHash64;
struct Struct {
    e: i32,
}
#[derive(Debug)]
struct File1 {
    name: String,
    data: Vec<u8>,
}
fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 8, 99, 10, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    let k = (-42_f32).sqrt();
    if !k.is_nan() {
        print!("k is:{}", k);
    }

    let f = 1.03323f32;

    print!("{:.2}", f);

    for i in 1..=5 {
        println!("{}", i);
    }

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{}+{}i", result.re, result.im);

    println!("{}", std::mem::size_of_val(&f));

    let kk = { 1 + 2 };

    println!("{}", kk);
    plus_or_minus(1_i32);

    let mut s = String::from("ddd");
    s.push_str(",dddd");
    takes_ownership(s.clone());

    println!("{}", s);

    let test_x = 5;
    let test_y = &test_x;
    assert_eq!(5, test_x);
    assert_eq!(5, *test_y);

    let str = String::from("Hello");
    let length = calculate_length(&str);

    println!("str :{},length:{}", str, length);

    let new_str = "ddddd";

    string_test();
    struct_test();
    test_enum();
    test_option();
    let r = test_match(Coin::AUD);

    println!("{}", r);
    test_if_let();
    let p = Point::new(1, 2);
    p.diff();
    let p11 = P { x: 1, y: 2 };

    println!("{}", p11.x());

    let xy1 = XY { x: 1, y: 2 };
    let xy2 = XY { x: 3, y: 4 };
    let xy3 = xy1 + xy2;

    println!("{}", xy3);

    let person = Person;
    person.fly();
    Pilot::fly(&person);
    test_vec();
    test_hashmap();
    test_life();
    test_panic();
    let s = test_readfile();
    match s {
        Ok(s) => println!("{}", s),
        Err(error) => println!("read file error"),
    }

    crate::test_mod::pr();
    println!("{:04}", 1);
    println!("Hello {:1$}!", "x", 5);

    test_unsafe();

    test_async();

    let mut foo = Foo;

    let mut loan = foo.mutate_and_share();
    foo.share();
    //println!("{:?}",loan)
    // let closure_slision = |x: &i32| -> &i32 { x };
    let closure_fn = fun(|x: &i32| -> &i32 { x });

    let r11 = &mut vec![1, 2, 3];
    let rr: &Vec<_> = &*r11;

    test_reborrow();
    let c_str: &'static str = "";

    test_static();

    let xx_ = 1;
    let sum = |y: i32| xx_ + y;
    let yy_ = sum(2);
    println!("--->{}", yy_);

    test_thread();

    test_thread_barrier();
    test_thread_local();
    test_source();
    test_mutex();
    test_callonce();
    test_communication();
    test_continue_rec();
    test_congest();
    test_flume();

    let mm = Mutex::new(1);

        
        let mut mmm = mm.lock().unwrap();
        *mmm = 3;
        drop(mmm);
    println!("{:?}",mm);



#[derive(Debug)]
struct MyBox(*mut u8);
unsafe impl Send for MyBox {}

    let p = MyBox(5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{:?}",p);
        println!("{:?}",*&p.0);
    });

    t.join().unwrap();


struct Vector(Vec<String>);

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
let v_test = Vector(vec![String::from("1"),String::from("2"),String::from("3")]);

println!("{}",v_test);


type Meters = u32;

let x: u32 = 5;
let y: Meters = 5;

println!("x + y = {}", x + y);

test_box();

let leak  =test_leak();
println!("{}",leak)

}

fn test_leak()-> &'static str{
    let mut str = String::new();
    str.push_str("sunshine");
    Box::leak(str.into_boxed_str())
}
fn test_box(){
    let a   = Box::new(33);
    println!("{}",a);

    let arr = [0;1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = vec![Box::new(1), Box::new(2)];
    let (f,s)  = (&arr[0],&arr[1]);

    println!("{}",**f);



}
fn test_flume(){
    let (x,y) = flume::unbounded();

    thread::spawn(move||{

       (0..10).for_each(|i|{
            x.send(i).unwrap();
       });
    });

    let result:u32 = y.iter().sum();
    println!("sum is:{}",result);


}

fn test_congest(){

    let (send, recv) = std::sync::mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...
    drop(send);
    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}
fn test_continue_rec(){
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            //thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    
    //println!("{:?}",tx);
    for received in rx {
        println!("Got: {}", received);
    }
}
fn test_communication(){
    let (s,r) = std::sync::mpsc::channel();

    thread::spawn(move||{
        s.send(123).unwrap();
    });


   let data =  r.recv().unwrap();

    println!("receive:{}",data);
}
fn test_callonce(){
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();
    let one = thread::spawn(||{

        INIT.call_once(||{
            unsafe{
                VAL=1;
            }
        });
    });

    let two = thread::spawn(||{

        INIT.call_once(||{
            unsafe{
                VAL=2;
            }
        });
    });

    one.join().unwrap();
    two.join().unwrap();
    println!("{:?}",unsafe {VAL});


}
fn test_mutex(){
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}
fn test_source() {
    let tls = Arc::new(thread_local::ThreadLocal::new());
    let mut v = vec![];
    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            // 将计数器加1
            // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
            // 这只能在线程退出后发生，因此不会导致任何竞争条件
            let cell = tls2.get_or(|| std::cell::Cell::new(0));
            cell.set(cell.get() + 1);
            
        });
        v.push(handle);
    }
    for handle in v {
        handle.join().unwrap();
    }
    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
        // 因为一些线程已退出，并且其他线程会回收退出线程的对象
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });
    println!("total->{}", total);
}
fn test_thread_local() {
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        *f.borrow_mut() = 2;
    });

    let t = thread::spawn(move || {
        FOO.with(|f| {
            println!("{}", f.borrow());
            *f.borrow_mut() = 3;
        });
    });

    t.join();

    FOO.with(|f| {
        println!("{}", f.borrow());
    });
}

fn test_thread_barrier() {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    for _ in 0..5 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for h in handles {
        h.join();
    }
}
fn test_thread() {
    let handler = std::thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    handler.join();
}
fn test_static() {
    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = &STATIC_EXAMPLE;
        let x = "&'static str";
        r2 = x;
        // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
    }

    println!("&'static i32: {}", r1); // -> 42
    println!("&'static str: {}", r2); // -> &'static str

    let r3: &str;

    {
        let s1 = "String".to_string();

        // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
        // 充分说明这个约束是多么的弱。。
        static_bound(&s1);

        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
        //r3 = &s1;

        // s1 在这里被 drop
    }
    //println!("{}", r3);
}

fn static_bound<T: Display + 'static>(t: &T) {
    println!("{}", t);
}
fn test_reborrow() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r;

    println!("{:?}", r);
    println!("{:?}", rr);
}
fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
    f
}

fn fn_elision(x: &i32) -> &i32 {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("{}", announcement);
        self.part
    }
}

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&mut self) {}
}

fn test_async() {
    let futures = do_something();
    futures::executor::block_on(futures);
    futures::executor::block_on(hello_world());
}
async fn do_something() {
    println!("do_something")
}

async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
}

fn test_unsafe() {
    let mut num = 5;
    let r = &num as *const i32;
    let r2 = &mut num as *const i32;

    unsafe {
        println!("{:?}", *r);
    }
    let a: Box<i32> = Box::new(10);
    println!("{:?}", &a);

    let (address, len) = get_str_address_len();

    let str = get_str_at_location(address, len);



    
    println!("add:{},len:{},value:{}", address, len, str);

    let a: Box<i32> = Box::new(10);

    let pointer: *const i32 = &*a;
    let c: *const i32 = Box::into_raw(a);
    println!("{}", pointer == c);
    unsafe {
        abs(1);
        std::arch::asm!("nop");
    }

    let x: u64;
    unsafe {
        std::arch::asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);
}
extern "C" {
    fn abs(input: i32) -> i32;
}
// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(pointer as *const u8, length))
    }
}

fn get_str_address_len() -> (usize, usize) {
    let str = "String::fromvalue";
    let address = str.as_ptr() as usize;

    let len = str.len();

    (address, len)
}

mod test_mod {

    pub fn pr() {
        println!("sunshine--com");
    }

    mod mod1 {

        fn pr() {
            super::pr();
        }
    }
}

fn test_readfile() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(f) => f,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(str_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn test_panic() {
    // panic!("error")

    let ip: Ipv4Addr = "127.0.0.1".parse().unwrap();

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("sss"),
            },
            other_error => panic!("ssss"),
        },
    };

    //let f_pointer = File::open("path").expect("msg");
}

fn test_life() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let a: &'static str = "sunshine";
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn test_hashmap() {
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();

    /**
    for k in &teams_list{
        teams_map.insert(&k.0, k.1);
    }
     */
    println!("{:?}", teams_map);

    let mut scores = HashMap::new();

    scores.insert(String::from("value"), 100);
    scores.insert(String::from("value2"), 200);

    for (k, v) in &scores {
        println!("{k}--{v}");
    }

    let b = scores.get(&String::from("value"));
}
fn test_vec() {
    let mut v = Vec::with_capacity(10);
    v.push(1);

    let mut k = vec![1, 2, 3, 4];
    let x = &k[2];

    k.sort();

    k.sort_by(|a, b| a.cmp(b));
}

struct Person;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

impl Pilot for Person {
    fn fly(&self) {
        println!("Pilot----");
    }
}

impl Wizard for Person {
    fn fly(&self) {
        println!("Wizard----");
    }
}

impl Person {
    fn fly(&self) {
        println!("person----");
    }
}

struct XY<T> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for XY<T> {
    type Output = XY<T>;
    fn add(self, xy: XY<T>) -> Self::Output {
        XY {
            x: self.x + xy.x,
            y: self.y + xy.y,
        }
    }
}

impl<T: Display> std::fmt::Display for XY<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sunshine {{ x: {}, y: {} }}", self.x, self.y)
    }
}

pub trait Summary {
    fn summary(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub content: String,
}

impl Summary for Post {
    fn summary(&self) -> String {
        format!("文章{}", self.title)
    }
}

pub struct Weibo {
    pub title: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summary(&self) -> String {
        format!("{}", self.title)
    }
}

fn test<T: Summary + Display>(p: &T) -> String {
    String::new()
}
struct P<T> {
    x: T,
    y: T,
}
impl<T> P<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    fn diff(&self) -> i32 {
        return self.x - self.y;
    }
}
fn test_if_let() {
    let a = Some(100);
    println!("{:?}", a);
    if let Some(a) = a {
        println!("{:?}", a);
    }

    println!("{:?}", a);
}

enum Coin {
    JP,
    CN,
    AUD,
}
fn test_match(coin: Coin) -> i32 {
    match coin {
        Coin::CN => 1,
        Coin::JP | Coin::AUD => 3,
        _ => 2,
    }
}

fn test_option() {
    let a = Some(5);
    let r = plus_one(a);
    println!("{:?}", r);
}

fn plus_one(data: Option<i32>) -> Option<i32> {
    match data {
        Some(i) => Some(i + 1),
        None => None,
    }
}

#[derive(Debug)]
enum PokerSuit {
    Hearts,
    Diamonds,
}
fn test_enum() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    println!("{:?}", heart);
    println!("{:?}", diamond);
}

fn string_test() {
    let mut str = String::from("hello,");
    str.push_str("world");
    println!("{}", str);

    str.insert_str(4, "abc");
    println!("{}", str);

    dbg!(str.pop());

    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    dbg!(s2);
    let s4 = format!("{}", s3);
    dbg!(s4);
    //dbg!(s4);
    tuple_test();

    let user = User {
        name: String::from("value"),
        age: 32,
        male: true,
    };
    //struct_test();
}

fn struct_test() {
    let f1 = File1 {
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

struct User {
    name: String,
    age: i32,
    male: bool,
}

fn tuple_test() {
    let tup = (1, 2, 3);
    let (x, y, z) = tup;

    print!("{}", x);
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn demo() {
    let datas = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

    let records = datas.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{},{}cm", name, length);
        }
    }
}

fn greet_world() {
    let chinese = "你好";
    let en = "Hello World";
    let southern_germany = "Grüß Gott!";
    let arr = [chinese, en, southern_germany];

    for hello in arr.iter() {
        println!("{}", hello);
    }

    let _x = 5;
}
