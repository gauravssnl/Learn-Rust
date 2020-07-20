//  Note : this code is just an overview, it won't compile

// Example of using the attribute like macro
#[route(GET, "/")]
fn index {
    // code snippets
}

// Macro definition overview
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item:TokenStream) -> TokenStream {
    // attr : refers to the 'GET, "/"' part
    // item refers to the body of the item attr is attached to; here fn index() {}
}