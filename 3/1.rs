


fn main(){
    println!("Hello world!");
   
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Not statically typed?
    // but mutable...
    let var1 = "hello"; 
    println!("{var1}");    

    // The second time a variable is defined with the let keyword while 
    // having the same name, it is being overshadowed (meaning pretty much 
    // remade.)
    let var1 = 10; 
    println!("{var1}");

}