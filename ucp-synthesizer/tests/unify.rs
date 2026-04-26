use ucp_core::cam::AbstractPropType;
use ucp_synthesizer::unify::map_raw_type_to_cam;

#[test]
fn map_leptos_rw_signal_to_controlled_value() {
    let cam_type = map_raw_type_to_cam("RwSignal < String >").unwrap();
    assert!(matches!(cam_type, AbstractPropType::ControlledValue(_)));
}

#[test]
fn map_plain_bool_to_control_flag() {
    let cam_type = map_raw_type_to_cam("bool").unwrap();
    assert_eq!(cam_type, AbstractPropType::ControlFlag);
}
