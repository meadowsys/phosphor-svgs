use roxmltree::Document as XmlDocument;
use std::mem;

pub struct Icon {
	pub path_d: String,
	pub path_d_duotone: Option<String>
}

impl Icon {
	pub fn parse(icon: &str) -> Self {
		// todo currently does not parse duotone
		let parsed = XmlDocument::parse(icon).expect("invalid icon");

		let root = parsed.root();
		assert_eq!(root.tag_name().name(), "", "sanity check");

		let mut root_children = root.children();
		let svg = root_children.next().expect("icon must have root item");
		assert!(svg.is_element(), "root item must be an element");
		assert_eq!(svg.tag_name().name(), "svg", "root element must be svg");
		assert_eq!(svg.tag_name().namespace(), Some("http://www.w3.org/2000/svg"), "missing svg xmlns");
		assert_eq!(svg.attributes().len(), 2, "extra attrs in svg tag found");
		assert_eq!(svg.attribute("viewBox"), Some("0 0 256 256"), "must have viewBox attr");
		assert_eq!(svg.attribute("fill"), Some("currentColor"), "must have fill attr");
		assert!(root_children.next().is_none(), "must have only one root element");

		let mut svg_children = svg.children();
		let path = svg_children.next().expect("svg must have at least one child");
		assert!(path.is_element(), "svg child must be element");
		assert_eq!(path.tag_name().name(), "path", "svg child must be path tag");
		assert_eq!(path.tag_name().namespace(), Some("http://www.w3.org/2000/svg"), "svg child path should have svg xmlns");
		let mut path_d = path.attribute("d").expect("path d attr must exist").into();
		let mut path_d_duotone = None;
		match path.attributes().len() {
			1 => {
				// other
			}
			2 => {
				// duotone, need special handling
				assert_eq!(path.attribute("opacity"), Some("0.2"), "first path in duotone icon must have opacity attr");
				path_d_duotone = Some(mem::take(&mut path_d));
				let path = svg_children.next().expect("svg must have two children in duotone icon");
				assert!(path.is_element(), "svg child 2 must be element");
				assert_eq!(path.tag_name().name(), "path", "svg child 2 must be path tag");
				assert_eq!(path.tag_name().namespace(), Some("http://www.w3.org/2000/svg"), "svg child 2 path should have svg xmlns");
				assert_eq!(path.attributes().len(), 1, "extra attrs in path tag found");
				path_d = path.attribute("d").expect("path d attr must exist").into();
			}
			_ => {
				// 0 can't happen, we just got an attribute before this match statement
				panic!("extra attrs in path tag found");
			}
		}
		assert!(svg_children.next().is_none(), "svg should have only one child");

		assert!(!path.has_children(), "path should have no children");

		Self { path_d, path_d_duotone }
	}
}
