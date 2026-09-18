use roxmltree::Document as XmlDocument;

pub struct Icon {
	path_d: String
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
		let path = svg_children.next().unwrap();
		assert!(path.is_element(), "svg child must be element");
		assert_eq!(path.tag_name().name(), "path", "svg child must be path tag");
		assert_eq!(path.tag_name().namespace(), Some("http://www.w3.org/2000/svg"), "path should have svg xmlns");
		assert_eq!(path.attributes().len(), 1, "extra attrs in path tag found");
		let path_d = path.attribute("d").expect("path d attr must exist").into();
		assert!(svg_children.next().is_none(), "svg should have only one child");

		assert!(!path.has_children(), "path should have no children");

		Self { path_d }
	}
}
