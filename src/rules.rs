use crate::value::Value;

pub fn checkers() -> Vec<Value> {
    vec![
        Value {
            value: String::from("w"),
            color: String::from("white"),
            connectors: [
                vec![String::from("1")],
                vec![String::from("2")],
                vec![String::from("3")],
                vec![String::from("4")],
            ]
        },Value {
            value: String::from("b"),
            color: String::from("black"),
            connectors: [
                vec![String::from("3")],
                vec![String::from("4")],
                vec![String::from("1")],
                vec![String::from("2")],
            ]
        },
    ]
}

pub fn terrain() -> Vec<Value> {
    vec![
        Value {
            value: String::from("."),
            color: String::from("darkblue"),
            connectors: [
                vec![String::from("dblue")],
                vec![String::from("dblue")],
                vec![String::from("dblue")],
                vec![String::from("dblue")],
            ]
        },
        Value {
            value: String::from("."),
            color: String::from("darkblue"),
            connectors: [
                vec![String::from("blue"), String::from("dblue")],
                vec![String::from("blue"), String::from("dblue")],
                vec![String::from("blue"), String::from("dblue")],
                vec![String::from("blue"), String::from("dblue")],
            ]
        },
        Value {
            value: String::from(":"),
            color: String::from("blue"),
            connectors: [
                vec![String::from("blue"), String::from("yellow")],
                vec![String::from("blue"), String::from("yellow")],
                vec![String::from("blue"), String::from("yellow")],
                vec![String::from("blue"), String::from("yellow")],
            ]
        },
        Value {
            value: String::from("o"),
            color: String::from("cornsilk"),
            connectors: [
                vec![String::from("yellow"), String::from("green")],
                vec![String::from("yellow"), String::from("green")],
                vec![String::from("yellow"), String::from("green")],
                vec![String::from("yellow"), String::from("green")],
            ]
        },
        Value {
            value: String::from("O"),
            color: String::from("green"),
            connectors: [
                vec![String::from("green")],
                vec![String::from("green")],
                vec![String::from("green")],
                vec![String::from("green")],
            ]
        },
    ]
}