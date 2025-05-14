use tera::{Context, Tera};

fn main() {
    // Create new tera instance with sample template
    let mut tera = Tera::default();
    tera.add_raw_template(
        "info",
        "My age is {{ p.age }} and weight is {{ p.weight }}kg.",
    )
    .expect("add template 'info'");
    tera.add_raw_template(
        "json",
        r#"{"age": {{ p.age }}, "weight": "{{ p.weight }}kg"}"#,
    )
    .expect("add template 'json'");

    let person = serde_json::json!({
      "p": {
        "age": 12,
        "weight": 30
      }
    });

    // Create new context
    let context = Context::from_value(person).expect("build ctx from json");

    // Render template using the context
    let output = tera.render("info", &context).unwrap();
    assert_eq!(output, "My age is 12 and weight is 30kg.");

    let output = tera.render("json", &context).unwrap();
    assert_eq!(output, r#"{"age": 12, "weight": "30kg"}"#);
}
