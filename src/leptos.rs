use self::util::{ Icon, IntoPaths as _, Weight };
use leptos::{ IntoView, component, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::html::ElementChild as _;
use leptos::reactive::wrappers::read::Signal;
use std::mem::MaybeUninit;

pub mod icons;
pub mod weights;
mod util;

/// Phosphor icon component
#[component]
pub fn Icon<I, W>(
	#[prop(default = I::INIT)]
	icon: I,
	#[prop(default = W::INIT)]
	weight: W,
	#[prop(optional, into)]
	duotone_opacity: Option<Signal<f32>>
) -> impl IntoView
where
	I: Icon,
	W: Weight
{
	const {
		let i = size_of::<I>();
		let w = size_of::<W>();
		assert!(i == 0 && w == 0, "sorgy i was curious");
	}

	// these are ZSTs and the values are only to enable attribute API
	let _ = (icon, weight);

	let icon_data = W::get_icon_data::<I>();
	let needs_opacity = icon_data.needs_opacity();
	let duotone_opacity = match duotone_opacity {
		// if it's passed in, it's unconditionally set
		Some(duotone_opacity) => { MaybeUninit::new(duotone_opacity) }
		// if it isn't passed in, it's set if needs_opacity
		None if needs_opacity => { MaybeUninit::new(Signal::stored(0.2)) }
		None => { MaybeUninit::uninit() }
	};

	// SAFETY: we checked the needs_opacity value and initialised it as required
	let paths = unsafe {
		icon_data.into_paths(
			duotone_opacity,
			// if needs_opacity is true it's always set
			// either via passed in param or default initialisation
			#[cfg(debug_assertions)]
			needs_opacity,
			#[cfg(debug_assertions)]
			std::any::type_name::<(I, W)>()
		)
	};

	view! {
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">
			{paths}
		</svg>
	}
}

/// Function to get whether or not a combination of icon and weight makes use of
/// the opacity parameter
///
/// Might not be public or stable or kept. Don't know. Don't rely on this.
#[doc(hidden)]
#[inline]
pub fn uses_opacity<I: Icon, W: Weight>() -> bool {
	W::get_icon_data::<I>().needs_opacity()
}
