// This chapter is nothing new, functions, parameters and return types
// are all not novel to me.

// chapter 3.4 is just comments and doc comments, also nothing special.

fn main(){

    println!("Hello world");
    println!("{}", func(5));
}

fn func(val: u32 /* wow a comment */) -> bool {
    println!("wowie, text from func {val}"); // wow another comment 
    // wow comments galore.
    true
}