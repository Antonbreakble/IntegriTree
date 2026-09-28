use integritree_core::{
    export, generate, parse_opc_connection, OpcConfig, OpcServerProfile,
    Signal, SignalKind,
};

#[test]
fn opc_profiles_generate_expected_node_ids() {
    let xml = r#"<root><Node Name="Parameters"><Prop_BSTR Id="0">1</Prop_BSTR><Prop_BSTR Id="1">1;</Prop_BSTR></Node></root>"#;
    let connection = parse_opc_connection(xml).unwrap();

    let signals = [Signal {
        name: "AI_Signal".into(),
        kind: SignalKind::Ai,
        description: String::new(),
    }];

    for (profile, expected) in [
        (
            OpcServerProfile::Regul,
            "NS2|String|Application.AIs.AI_Signal.OUT.PV",
        ),
        (
            OpcServerProfile::Codesys,
            "NS4|String||var|CODESYS Control Win V3 x64.Application.AIs.AI_Signal.OUT.PV",
        ),
    ] {
        let config = OpcConfig::for_profile(connection, profile);
        let csv = export(&generate(&signals, &config).unwrap());

        let expected_line = format!("AIs.AI_Signal.OUT.PV,7996,vt_bstr,1#1###-1#{expected}");

        assert!(
            csv.lines().any(|line| line == expected_line),
            "не найден источник {expected} для {profile:?}"
        );
    }
}