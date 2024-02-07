use ::std::{ fs, path };
use ::hashbrown::HashMap;
const ROOT: &str = "phosphor-core/assets";

fn main() {
	let mut root_dir = path::PathBuf::new();
	root_dir.push("..");
	root_dir.push(ROOT);

	let mut styles = fs::read_dir(&root_dir)
		.unwrap()
		.map(|dirent| dirent.unwrap().file_name().to_str().unwrap().to_string())
		.collect::<Vec<_>>();
	styles.sort_unstable();

	let entries = styles.iter()
		.map(|style| {
			let style = &**style;
			let mut dir_path = path::PathBuf::from(&root_dir);
			dir_path.push(style);
			(style, dir_path)
		})
		.flat_map(|(style, dir_path)| {
			let suffix = if style == "regular" { ".svg".into() } else { format!("-{style}.svg") };
			let mut icons = fs::read_dir(dir_path).unwrap()
				.map(|ent| ent.unwrap().file_name().to_str().unwrap().to_string())
				.map(|filename| filename.strip_suffix(&suffix).unwrap().into())
				.collect::<Vec<String>>();
			icons.sort();
			[style].into_iter()
				.cycle()
				.zip(icons)
				.zip([suffix].into_iter().cycle())
				.map(|((style, icon), suffix)| (style, icon, suffix))
		})
		.collect::<Vec<_>>();

	let grouped_by_style = {
		let mut grouped = HashMap::<_, Vec<_>>::new();

		for (style, icon, suffix) in &entries {
			let entry = grouped
				.entry(*style)
				.or_default();
			entry.push((&**icon, &**suffix));
		}

		grouped
	};

	let grouped_by_icon = {
		let mut grouped = HashMap::<_, Vec<_>>::new();

		for (style, icon, suffix) in &entries {
			let entry = grouped
				.entry(&**icon)
				.or_default();
			entry.push((*style, &**suffix));
		}

		grouped
	};

	let mut grouped_by_style = grouped_by_style.into_iter().collect::<Vec<_>>();
	grouped_by_style.sort();

	let mut grouped_by_icon = grouped_by_icon.into_iter().collect::<Vec<_>>();
	grouped_by_icon.sort();

	let style = {
		let mut style = Vec::new();
		style.push("pub mod style {".into());

		for (s, icons) in grouped_by_style {
			style.push(format!("\tpub mod {s} {{"));

			for (i, suffix) in icons {
				let i_uppername = i.chars()
					.map(|c| if c == '-' { '_' } else { c })
					.flat_map(|c| c.to_uppercase())
					.collect::<String>();
				style.push(format!("\t\tpub const {i_uppername}: &str = include_str!(\"../{ROOT}/{s}/{i}{suffix}\");"));
			}

			style.push("\t}".into());
		}

		style.push("}".into());

		style
	};

	let icon = {
		let mut icon = Vec::new();
		icon.push("pub mod icon {".into());

		for (i, style) in grouped_by_icon {
			let i_underscored = i.chars()
				.map(|c| if c == '-' { '_' } else { c })
				.collect::<String>();
			icon.push(format!("\tpub mod {i_underscored} {{"));

			for (s, suffix) in style {
				let s_uppercase = s.to_uppercase();
				icon.push(format!("\t\tpub const {s_uppercase}: &str = include_str!(\"../{ROOT}/{s}/{i}{suffix}\");"));
			}

			icon.push("\t}".into());
		}

		icon.push("}".into());

		icon
	};

	let file = style
		.into_iter()
		.chain(["", ""].map(Into::into))
		.chain(icon)
		.fold(String::new(), |mut acc, curr| {
			acc.push_str(&curr);
			acc.push('\n');
			acc
		});

	// running in the scripts dir
	fs::write("../src/lib.rs", file).unwrap();
}
