//! `parse_tests.rs` is generated by build.rs
use ezxml::Node;

#[test]
fn bad_syntax_unescaped_angle_test() {
    let info = xtest::get_test_info("unescaped-angle");
    let xml_str = info.read_xml_file();
    let parse_result = ezxml::parse_str(xml_str.as_str());
    assert!(parse_result.is_err());
    let err = parse_result.err().unwrap();
    match err {
        ezxml::error::Error::Parse { position, .. } => {
            assert_eq!(position.absolute, 110);
            assert_eq!(position.line, 5);
            assert_eq!(position.column, 5);
        }
        _ => panic!("Error was expected to be of type ezxml::error::Error::Parse, but was not."),
    }
}

#[test]
fn good_syntax_difficult_nodes_test() {
    let info = xtest::get_test_info("difficult-nodes");
    let _xml_str = info.read_xml_file();
    // TODO - assert goodness
}

#[test]
fn bad_syntax_angle_in_attribute_value_test() {
    let info = xtest::get_test_info("angle-in-attribute-value");
    let xml_str = info.read_xml_file();
    let parse_result = ezxml::parse_str(xml_str.as_str());
    assert!(parse_result.is_err());
    let err = parse_result.err().unwrap();
    match err {
        ezxml::error::Error::Parse { position, .. } => {
            assert_eq!(position.absolute, 51);
            assert_eq!(position.line, 2);
            assert_eq!(position.column, 12);
        }
        _ => panic!("Error was expected to be of type ezxml::error::Error::Parse, but was not."),
    }
}

#[test]
fn good_syntax_ezfile_test() {
    let info = xtest::get_test_info("ezfile");
    let _xml_str = info.read_xml_file();
    // TODO - assert goodness
}

#[test]
fn bad_syntax_pi_stray_text_test() {
    let info = xtest::get_test_info("pi-stray-text");
    let xml_str = info.read_xml_file();
    let parse_result = ezxml::parse_str(xml_str.as_str());
    assert!(parse_result.is_err());
    let err = parse_result.err().unwrap();
    match err {
        ezxml::error::Error::Parse { position, .. } => {
            assert_eq!(position.absolute, 85);
            assert_eq!(position.line, 3);
            assert_eq!(position.column, 39);
        }
        _ => panic!("Error was expected to be of type ezxml::error::Error::Parse, but was not."),
    }
}
