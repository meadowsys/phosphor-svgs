use leptos::{ IntoView, component, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::either::Either;
use leptos::html::ElementChild as _;
use leptos::reactive::wrappers::read::Signal;

#[component]
pub fn Icon<I, W>(
	#[prop(optional)]
	icon: I,
	#[prop(optional)]
	weight: W,
	#[prop(into, default = Signal::stored(0.2))]
	duotone_opacity: Signal<f32>
) -> impl IntoView
where
	I: Default,
	W: Default,
	(I, W): GetIconData
{
	let _ = (icon, weight);

	view! {
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="currentColor">
			{<(I, W)>::get().into_paths(duotone_opacity)}
		</svg>
	}
}

pub struct IconPath {
	pub(super) d: &'static str,
	pub(super) has_opacity: bool
}

#[derive(Default)]
pub struct Bold;

#[derive(Default)]
pub struct Duotone;

#[derive(Default)]
pub struct Fill;

#[derive(Default)]
pub struct Light;

#[derive(Default)]
pub struct Regular;

#[derive(Default)]
pub struct Thin;

pub trait Icon {
	fn bold_data() -> impl IconWeightData;
	fn duotone_data() -> impl IconWeightData;
	fn fill_data() -> impl IconWeightData;
	fn light_data() -> impl IconWeightData;
	fn regular_data() -> impl IconWeightData;
	fn thin_data() -> impl IconWeightData;
}

pub trait GetIconData {
	fn get() -> impl IconWeightData;
}

impl<I: Icon> GetIconData for (I, Bold) {
	#[inline]
	fn get() -> impl IconWeightData {
		I::bold_data()
	}
}

impl<I: Icon> GetIconData for (I, Duotone) {
	#[inline]
	fn get() -> impl IconWeightData {
		I::duotone_data()
	}
}

impl<I: Icon> GetIconData for (I, Fill) {
	#[inline]
	fn get() -> impl IconWeightData {
		I::fill_data()
	}
}

impl<I: Icon> GetIconData for (I, Light) {
	#[inline]
	fn get() -> impl IconWeightData {
		I::light_data()
	}
}

impl<I: Icon> GetIconData for (I, Regular) {
	#[inline]
	fn get() -> impl IconWeightData {
		I::regular_data()
	}
}

impl<I: Icon> GetIconData for (I, Thin) {
	#[inline]
	fn get() -> impl IconWeightData {
		I::thin_data()
	}
}

pub trait IconWeightData {
	fn into_paths(self, opacity: Signal<f32>) -> impl IntoView;
}

impl<const N: usize> IconWeightData for [IconPath; N] {
	#[inline]
	fn into_paths(self, opacity: Signal<f32>) -> impl IntoView {
		self.map(|IconPath { d, has_opacity }| {
			if has_opacity {
				Either::Left(view! { <path d=d opacity=opacity /> })
			} else {
				Either::Right(view! { <path d=d /> })
			}
		})
	}
}
