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

	#[doc(hidden)]
	fn default() -> Self;
}

pub trait Weight {
	fn get_icon_data<I: Icon>() -> impl IconWeightData;
	#[doc(hidden)]
	fn default() -> Self;
}

impl Weight for Bold {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::bold_data()
	}

	#[inline]
	fn default() -> Self { Self }
}

impl Weight for Duotone {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::duotone_data()
	}

	#[inline]
	fn default() -> Self { Self }
}

impl Weight for Fill {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::fill_data()
	}

	#[inline]
	fn default() -> Self { Self }
}

impl Weight for Light {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::light_data()
	}

	#[inline]
	fn default() -> Self { Self }
}

impl Weight for Regular {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::regular_data()
	}

	#[inline]
	fn default() -> Self { Self }
}

impl Weight for Thin {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::thin_data()
	}

	#[inline]
	fn default() -> Self { Self }
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
