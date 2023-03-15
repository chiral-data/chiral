pub mod traits;
pub mod logging;
pub mod kinds;
pub mod utils;
pub mod job;
pub mod command;
pub mod app;
pub mod data;

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use chiral_derive::Serialization;
    use crate::traits::{Serialization, SerializedFormat};

    #[derive(Serialize, Deserialize, Serialization, PartialEq, Debug)]
    struct TestStruct {
        name: String
    }

    #[test]
    fn test_serialization() {
        let ts = TestStruct { name: "hello".to_string() };
        assert_eq!(ts.ser_to(), r#"{"name":"hello"}"#.to_string());
        let ts_desser = TestStruct::ser_from(&r#"{"name":"hello"}"#.to_string());
        assert_eq!(ts_desser, ts);
    }
}