//this program returns the first word from a string
fn main (){
    let name=String::from("Amschel kariuki");
    let first_name=first_word(&name);
    println!("The first name is {}",first_name);

}
fn first_word(s:&String)->&str{
let bytes= s.as_bytes();
for (i, &element) in bytes.iter().enumerate(){
    if element== b' '{
        return &s[0..i];

    }
}
&s[..]
}
