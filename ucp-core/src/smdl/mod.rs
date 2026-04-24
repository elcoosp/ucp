mod parser;

pub fn parse_smdl(input: &str) -> crate::Result<serde_json::Value> {
    parser::parse_smdl_internal(input)
}
