use tera::{Context, Tera};

fn main() {
    // Create new tera instance with sample template
    let mut tera = Tera::default();
    tera.add_raw_template("info", "My age is {{ age }}.").expect("add template 'info'");

    // Create new context
    let mut context = Context::new();
    context.insert("age", &18);

    // Render template using the context
    let output = tera.render("info", &context).unwrap();
    assert_eq!(output, "My age is 18.");
}
