use std::path::PathBuf;

const XML_FILES: &str = "xml-files";
const XML_DATA: &str = "data";

/// Returns the path to this crate.
fn self_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap()
}

/// Assumes that we are in a crate within a workspace and returns the path to the workspace dir.
pub fn workspace_dir() -> PathBuf {
    let mut p = self_dir();
    p.pop();
    p.canonicalize().unwrap()
}

pub fn xml_data_dir() -> PathBuf {
    workspace_dir().join(XML_DATA).canonicalize().unwrap()
}

// pub fn xml_syntax_errors() -> PathBuf {
//     xml_data_dir().join(XML_SYNTAX_ERRORS).canonicalize().unwrap()
// }

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{self_dir, workspace_dir, xml_data_dir, xml_syntax_errors};

    #[test]
    fn test_self_dir() {
        let dir = self_dir();
        let components = dir.components();
        let last_component = components.last().unwrap().as_os_str().to_string_lossy();
        assert!(last_component.as_ref() == "test-dir");
    }

    #[test]
    fn test_xml_dir() {
        let dir = xml_data_dir();
        assert!(Path::new(&dir).is_dir());
        let components = dir.components();
        let last_component = components.last().unwrap().as_os_str().to_string_lossy();
        assert!(last_component.as_ref() == super::XML_FILES);
    }

    #[test]
    fn test_xml_syntax_errors_dir() {
        let dir = xml_syntax_errors();
        assert!(Path::new(&dir).is_dir());
        let components = dir.components();
        let last_component = components.last().unwrap().as_os_str().to_string_lossy();
        assert!(last_component.as_ref() == super::XML_SYNTAX_ERRORS);
    }
}
