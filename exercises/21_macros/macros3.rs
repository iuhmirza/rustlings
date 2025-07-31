// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    
    pub use my_macro;
}

fn main() {
    macros::my_macro!();
}
