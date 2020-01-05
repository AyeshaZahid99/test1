/*fn main() {
    let f=vec![40,20,30,40];
    let  mut g=&f[0];
    for item in f.iter(){
        if item>&f[0]{
            g=item;
        }
    }
    println!("{:?}",g );

    let p=vec![20,300,200,8000,90,900000];
    let mut q=&p[0];
    for  element in p.iter(){
        if element>&p[0]{
            q=element;
        }
        
    }
    println!("{}",q );
}
    /////////////////////////////////
    

 fn largest1(list:&[i32])->i32{
     let mut largest=list[0];
     for &items in list.iter(){
         if items>list[0]{
             largest=items
         }
     }
     largest
 }
 fn largest2(list:&[char])->char{
     let mut largesst=list[0];
     for &element in list.iter(){
         if element>list[0]{
             largesst=element;
         }
     }
     largesst
 }
 fn main(){
     let ops=vec![23,45,56,67,677,5678];
     let result=largest1(&ops);
     println!("{}",result );

     let ops1=vec!['a','z','c','d'];
     let result1=largest2(&ops1);
     println!("{}",result1 );
 }
      
 /////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Point<T,U>{
    x: T,
    y: U,
}
impl<T,U>Point<T,U>{
    fn mac(self)->(U){
        self.y
    }
}
    fn main(){
        let ins=Point{
        x:50,
        y:60
        };
        print!("{}",ins.mac() );
    }
    */ 
    /*
    struct Apple<T,U>{
         x:T,
         y:U,
    }
    impl Apple<f64,u8>{
        fn mango(self)->f64{
            self.x.powi(2)+ self.y.powi(2)
        }
    }
    fn main(){
        let inst=Apple{
            x:20.0,
            y:30,
        };
        println!("{}",inst.mango() );
    }
*/


/*

fn main() {
    struct Point<T,U> {
        x: T,
        y: U,
    }
    
    impl Point<f32,f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let ins1=Point{
        x:20.0,
        y:30.0,
    };
    println!("{}",ins1.distance_from_origin() );
    }
*/


pub trait Summary{
    fn summarize(&self)->String;
}
pub struct NewsArticle{
     pub author:String,
     pub content:String,
}
pub struct Tweet{
    pub username:String,
    pub content:String,
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{} :{}",self.author,self.content)
    }    
}
impl Summary for Tweet{
    fn summarize(&self)->String{
        format!("{}:{}",self.username,self.content)
    }
}
fn main(){
    let inst1=NewsArticle{
        author:String::from("Khalid Hosseni"),
        content:String::from("The most reknowed writer!!"),
    };
    let inst2=Tweet{
        username:String::from("Eliff"),
        content:String::from("Another woman having a realistic approach!!"),
    };
println!("{}", inst1.summarize());
println!("{}", inst2.summarize());

// }
