mod smdl;

pub fn parse_smdl(input: &str) -> crate::Result<serde_json::Value> {
    smdl::parse_smdl_internal(input)
}
