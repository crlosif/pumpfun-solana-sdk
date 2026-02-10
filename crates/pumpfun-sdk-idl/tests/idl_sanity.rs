use pumpfun_sdk_idl::{embedded, idl::Idl, util};

#[test]
fn parse_embedded_idl() {
    let json = embedded::pump_bonding_curve_min_idl_json();
    let idl: Idl = serde_json::from_str(json).expect("valid json");
    assert!(idl.instructions.iter().any(|ix| ix.name == "create_v2"));
}

#[test]
fn discriminator_matches_anchor_rule() {
    let expected = util::anchor_global_discriminator("create_v2");
    let actual = pumpfun_sdk_idl::generated::pump_bonding_curve_min::create_v2::DISCRIMINATOR;
    assert_eq!(expected, actual);
}
