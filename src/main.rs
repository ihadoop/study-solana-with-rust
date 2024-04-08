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
