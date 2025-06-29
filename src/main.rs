use rijson::parser::Parser;

fn main() {
    let input = String::from(
        r#"[
      {
        "name": "Michael",
        "age": 30,
        "count": 1e-10
      },
      {
        "name": "Kelly",
        "age": 25,
        "count": 1.10
      },

      {
        "name": "John",
        "age": 35,
        "count": 1.0e+10


      },
      {

        "name": "Alice",
        "age": 28,

        "count": 1.0
      },
      {
        "name": "Bob",
        "age": 22,
        "count": null,
        "address": {
          "city": "New York",
          "zip": "10001"
        },
        "status": "pending"
      },
      {

        "name": "Charlie",
        "age": 40,
        "count": 1.0e+5,


        "address": {
          "city": "Los Angeles",
          "zip": "90001"

        }

      },
      {
        "name": "Diana",
        "age": 32,
        "count": 1.0e-3,
        "address": {
          "city": "Chicago",
          "zip": "60601"
        }
      }
    ]"#,
    );
    let p = Parser::new(input.chars().collect());
    for i in p {
        println!("{}", i.unwrap())
    }
}
