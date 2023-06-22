fn main (){
    let s= String::from("hello");
    let x=34;
  use_string(s);
  use_int(x);
  println!("do you know x is {}",x);
  //println!("do you know that s is {}",s);//cannot happen
}
fn use_string(mystr:String){
    println!("{}",mystr);
}
fn use_int(my_int:u32){
    println!("{}",my_int);
}