fn main(){
    let s= String::from("hello");
    let r1=&mut s;
    let r2= $mut s; // will bring an error
    //cannot borrow `s` as mutable more than once at a time
    //also cannot borrow s as mutable if its borrowed as immutable
}