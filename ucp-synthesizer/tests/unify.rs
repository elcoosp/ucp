use ucp_core::cam::AbstractPropType;
use ucp_synthesizer::unify::map_raw_type_to_cam;

#[test]
fn map_leptos_rw_signal_to_controlled_value() {
    let raw_type = "RwSignal < String >".to_string();
    let cam_type = map_raw_type_to_cam(&raw_type).unwrap();
    assert!(matches!(cam_type, AbstractPropType::ControlledValue(_)));
}

#[test]
fn map_plain_bool_to_control_flag() {
    let raw_type = "bool".to_string();
    let cam_type = map_raw_type_to_cam(&raw_type).unwrap();
    assert_eq!(cam_type, AbstractPropType::ControlFlag);
}
