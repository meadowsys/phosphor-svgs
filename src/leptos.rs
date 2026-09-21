use self::util::{ Icon, IconWeightData as _, Weight };
use leptos::{ IntoView, component, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::html::ElementChild as _;
use leptos::reactive::wrappers::read::Signal;

pub mod icons;
pub mod weights;
mod util;

#[component]
pub fn Icon<I, W>(
	#[prop(default = I::default())]
	icon: I,
	#[prop(default = W::default())]
	weight: W,
	#[prop(into, default = Signal::stored(0.2))]
	duotone_opacity: Signal<f32>
) -> impl IntoView
where
	I: Icon,
	W: Weight
{
	let _ = (icon, weight);
	let paths = W::get_icon_data::<I>()
		.into_paths(duotone_opacity);

	view! {
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="currentColor">
			{paths}
		</svg>
	}
}
