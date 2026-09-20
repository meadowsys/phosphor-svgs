use super::{ Bold, Duotone, Fill, Light, Regular, Thin };
use leptos::{ IntoView, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::either::Either;
use leptos::reactive::wrappers::read::Signal;

pub struct IconPath {
	pub(super) d: &'static str,
	pub(super) has_opacity: bool
}

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
