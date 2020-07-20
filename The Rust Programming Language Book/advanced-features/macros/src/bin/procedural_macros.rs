// Note : this code won;t compile , just an overview
// When creating procedural macros, the definitions must reside in their own crate with a special crate type. 
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {

}