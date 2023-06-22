
fn main(){
    let name= String::from("Amschel kariuki");
    let fname=first_word(&name);
    println!("The first name is, {}",fname);
}
fn first_word(s:&String)->&str{
//Convert the string to an array of iterable bytes
let bytes= s.as_bytes();
for (i,&item) in bytes.iter().enumerate(){
    if item == b' '{
        return &s[0..i];
    }
}
&s[..]
    

}
