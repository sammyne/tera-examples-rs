use tera::{Context, Tera};

fn main() {
    json_array_cannot_be_ctx();
    array_index_cannot_be_negative();

    // Create new tera instance with sample template
    let mut tera = Tera::default();
    tera.add_raw_template(
        "json",
        r#"{% for v in data %} {{ v.a }}--{{ v.b }}{% endfor %}"#,
    )
    .expect("add template 'json'");
    tera.add_raw_template("array[i]", r#"{{ data[1].a }}--{{ data[1].b }}"#)
        .expect("add template 'array[i]'");

    let person = serde_json::json!({
      "data": [
        {"a":1,"b":"hello"},
        {"a":2,"b":"world"}
      ]
    });

    // Create new context
    let context = Context::from_value(person).expect("build ctx from json");

    // Render template using the context
    let output = tera.render("json", &context).unwrap();
    assert_eq!(output, r#" 1--hello 2--world"#);

    let output = tera.render("array[i]", &context).unwrap();
    assert_eq!(output, r#"2--world"#);
}

fn json_array_cannot_be_ctx() {
    let json = serde_json::json!([
      {"a":1,"b":"hello"},
      {"a":2,"b":"world"}
    ]);

    // Create new context
    let _err = Context::from_value(json).expect_err("shouldn't build ctx from json");
}

fn array_index_cannot_be_negative() {
    // Create new tera instance with sample template
    let mut tmpl = Tera::default();
    tmpl.add_raw_template("array[i]", r#"{{ data[-1].a }}--{{ data[-1].b }}"#)
        .expect("add template 'array[i]'");

    let data = serde_json::json!({
      "data": [
        {"a":1,"b":"hello"},
        {"a":2,"b":"world"}
      ]
    });

    // Create new context
    let context = Context::from_value(data).expect("build ctx from json");

    let _err = tmpl
        .render("array[i]", &context)
        .expect_err("should error out for negative array index");
}
