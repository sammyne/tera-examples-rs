use tera::Context;

fn main() {
    let json = serde_json::json!([
      {"a":1,"b":"hello"},
      {"a":2,"b":"world"},
    ]);

    // Create new context
    let _err = Context::from_value(json).expect_err("shouldn't build ctx from json");
}
