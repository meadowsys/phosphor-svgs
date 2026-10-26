use self::util::{ Icon, IntoPaths as _, Weight };
use leptos::{ IntoView, component, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::html::ElementChild as _;
use leptos::reactive::wrappers::read::Signal;
use std::mem::MaybeUninit;

pub mod icons;
pub mod weights;
mod util;

#[component]
pub fn Icon<I, W>(
	#[prop(default = I::default())]
	icon: I,
	#[prop(default = W::default())]
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
	let duotone_opacity = match duotone_opacity {
		Some(duotone_opacity) => { MaybeUninit::new(duotone_opacity) }
		None if icon_data.needs_opacity() => { MaybeUninit::new(Signal::stored(0.2)) }
		None => { MaybeUninit::uninit() }
	};
	// SAFETY: we checked the needs_opacity value and initialised it as required
	let paths = unsafe { icon_data.into_paths(duotone_opacity) };

	view! {
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">
			{paths}
		</svg>
	}
}
