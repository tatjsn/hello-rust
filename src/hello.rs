use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
pub struct HelloTemplate<'a> { // the name of the struct can be anything
    pub name: &'a str, // the field name should match the variable name
                   // in your template
}