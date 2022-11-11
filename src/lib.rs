extern crate proc_macro;

use proc_macro::TokenStream;

const lorem_ipsum_str: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[proc_macro]
pub fn lorem_ipsum(item: TokenStream) -> TokenStream {
    let n: usize = item.to_string().parse().unwrap();
    let mut string = String::with_capacity(lorem_ipsum_str.len()*n+2);
    string.push('"');
    for _ in 0..=n {
        string.push_str(lorem_ipsum_str);
    }
    string.push('"');
    string.as_str().parse().unwrap()
}

#[proc_macro]
pub fn lorem_ipsum_nl(item: TokenStream) -> TokenStream {
    let n: usize = item.to_string().parse().unwrap();
    let mut string = String::with_capacity((lorem_ipsum_str.len() + 1)*n+2);
    string.push('"');
    for _ in 0..=n {
        string.push_str(lorem_ipsum_str);
        string.push('\n')
    }
    string.push('"');
    string.as_str().parse().unwrap()
}
