use self::util::{ GetIconData, IconWeightData as _ };
use leptos::{ IntoView, component, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::html::ElementChild as _;
use leptos::reactive::wrappers::read::Signal;

pub mod icons;
mod util;

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
