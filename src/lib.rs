
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro]
pub fn make_hello(item: TokenStream) -> TokenStream {
    let name = item.to_string();
    let hello = "Hello ".to_string() + name.as_ref();
    let func = "fn hello_".to_string() + name.as_ref() + "() { println!(\"" + hello.as_ref() + "\"); }";
    func.parse().unwrap()
}
