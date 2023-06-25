use crate::object::GQLObject;
use crate::types::DataType;

use lazy_static::lazy_static;
use std::collections::HashMap;

type Aggregation = fn(&String, usize, &Vec<GQLObject>) -> String;

pub struct AggregationPrototype {
    pub parameter: DataType,
    pub result: DataType,
}

lazy_static! {
    pub static ref AGGREGATIONS: HashMap<&'static str, Aggregation> = {
        let mut map: HashMap<&'static str, Aggregation> = HashMap::new();
        map.insert("max", aggregation_max);
        map
    };
}

lazy_static! {
    pub static ref AGGREGATIONS_PROTOS: HashMap<&'static str, AggregationPrototype> = {
        let mut map: HashMap<&'static str, AggregationPrototype> = HashMap::new();
        map.insert(
            "max",
            AggregationPrototype {
                parameter: DataType::Number,
                result: DataType::Number,
            },
        );
        map
    };
}

fn aggregation_max(field_name: &String, index: usize, objects: &Vec<GQLObject>) -> String {
    let mut max_length: i64 = 0;
    for i in 0..index + 1 {
        let object = &objects[i];
        let field_value = &object.attributes.get(field_name).unwrap();
        let int_value = field_value.parse::<i64>().unwrap();
        if int_value > max_length {
            max_length = int_value;
        }
    }
    return max_length.to_string();
}