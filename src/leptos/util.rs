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

pub trait IntoPath {
	fn into_path(self, opacity: Signal<f32>) -> impl IntoView;
}

impl IntoPath for Path {
	#[inline(always)]
	fn into_path(self, opacity: Signal<f32>) -> impl IntoView {
		let _ = opacity;
		let Self { d } = self;

		view! {
			<path d=d />
		}
	}
}

impl IntoPath for PathOpacity {
	#[inline(always)]
	fn into_path(self, opacity: Signal<f32>) -> impl IntoView {
		let Self { d } = self;

		view! {
			<path d=d opacity=opacity />
		}
	}
}

pub trait IntoPaths {
	fn into_paths(self, opacity: Signal<f32>) -> impl IntoView;
}

impl<P> IntoPaths for P
where
	P: IntoPath
{
	#[inline(always)]
	fn into_paths(self, opacity: Signal<f32>) -> impl IntoView {
		self.into_path(opacity)
	}
}

macro_rules! gen_into_paths {
	{ $($p:ident)* } => {
		impl<$($p),*> IntoPaths for ($($p),*)
		where
			$($p: IntoPath),*
		{
			#[inline(always)]
			fn into_paths(self, opacity: Signal<f32>) -> impl IntoView {
				#[expect(non_snake_case, reason = "macro")]
				let ($($p),*) = self;
				($($p.into_path(opacity)),*)
			}
		}
	}
}

gen_into_paths! { P1 P2 }
gen_into_paths! { P1 P2 P3 }
gen_into_paths! { P1 P2 P3 P4 }
gen_into_paths! { P1 P2 P3 P4 P5 }
