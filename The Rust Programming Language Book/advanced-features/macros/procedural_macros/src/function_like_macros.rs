// Note the following code won't compile. This is just an overview.
// Function-like macros define macros that look like function calls. 
// they can take an unknown number of arguments.

// Example 
let sql = sql!(SELECT * from student where student_id=1010);

// Macro definition overview
// This macro would parse the SQL statement inside it and check that it’s syntactically correct, 
// which is much more complex processing than a macro_rules! macro can do.

#[proc_macro]
pub fn sql(input TokenStream) -> TokenStream {
    // code snippets
}
