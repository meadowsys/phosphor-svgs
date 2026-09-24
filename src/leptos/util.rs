use leptos::{ IntoView, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::reactive::wrappers::read::Signal;

pub trait Icon {
	fn bold_data() -> impl IntoPaths;
	fn duotone_data() -> impl IntoPaths;
	fn fill_data() -> impl IntoPaths;
	fn light_data() -> impl IntoPaths;
	fn regular_data() -> impl IntoPaths;
	fn thin_data() -> impl IntoPaths;

	#[doc(hidden)]
	fn default() -> Self;
}

pub trait Weight {
	fn get_icon_data<I: Icon>() -> impl IntoPaths;

	#[doc(hidden)]
	fn default() -> Self;
}

pub struct Path {
	d: &'static str
}

pub struct PathOpacity {
	d: &'static str
}

#[inline(always)]
pub(super) fn p(d: &'static str) -> Path {
	Path { d }
}

#[inline(always)]
pub(super) fn po(d: &'static str) -> PathOpacity {
	PathOpacity { d }
}

pub trait IntoPaths {
	fn into_paths(self, opacity: Signal<f32>) -> impl IntoView;
}

impl IntoPaths for Path {
	#[inline(always)]
	fn into_paths(self, opacity: Signal<f32>) -> impl IntoView {
		let _ = opacity;
		let Self { d } = self;

		view! {
			<path d=d />
		}
	}
}

impl IntoPaths for PathOpacity {
	#[inline(always)]
	fn into_paths(self, opacity: Signal<f32>) -> impl IntoView {
		let Self { d } = self;

		view! {
			<path d=d opacity=opacity />
		}
	}
}

macro_rules! gen_into_path {
	{ $($p:ident)* } => {
		impl<$($p),*> IntoPaths for ($($p),*)
		where
			$($p: IntoPaths),*
		{
			#[inline(always)]
			fn into_paths(self, opacity: Signal<f32>) -> impl IntoView {
				#[expect(non_snake_case, reason = "macro")]
				let ($($p),*) = self;
				($($p.into_paths(opacity)),*)
			}
		}
	}
}

gen_into_path! { P1 P2 }
gen_into_path! { P1 P2 P3 }
gen_into_path! { P1 P2 P3 P4 }
gen_into_path! { P1 P2 P3 P4 P5 }
