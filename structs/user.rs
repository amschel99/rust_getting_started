struct User{
    email:String,
    f_name:String,
    l_name:String
}

fn main(){
    
   let user1= User{
        email:String::from("kariukiasmchel9@gmail.com"),
        f_name:String::from("Amschel"),
        l_name:String::from("Kariuki")
    };
    println!("The user  email is ,{}",user1.email);
}