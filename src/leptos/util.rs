use leptos::{ IntoView, view };
use leptos::attr::custom::CustomAttribute as _;
use leptos::reactive::wrappers::read::Signal;
use std::mem::MaybeUninit;

pub trait Icon {
	#[doc(hidden)]
	const INIT: Self;

	fn bold_data() -> impl IntoPaths;
	fn duotone_data() -> impl IntoPaths;
	fn fill_data() -> impl IntoPaths;
	fn light_data() -> impl IntoPaths;
	fn regular_data() -> impl IntoPaths;
	fn thin_data() -> impl IntoPaths;
}

pub trait Weight {
	#[doc(hidden)]
	const INIT: Self;

	fn get_icon_data<I: Icon>() -> impl IntoPaths;
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

/// # Safety
///
/// See safety docs on [`NEEDS_OPACITY`] and [`into_paths`]
///
/// [`NEEDS_OPACITY`]: IntoPaths::NEEDS_OPACITY
/// [`into_paths`]: IntoPaths::into_paths
pub unsafe trait IntoPaths {
	/// Variable declaring if this type reads the `opacity` parameter passed to
	/// [`into_paths`](IntoPaths::into_paths)
	///
	/// # Safety
	///
	/// For implementors: if you read the `opacity` parameter, you must set this
	/// to true.
	///
	/// For consumers: if this is set to true, you must initialise the `opacity`
	/// parameter.
	const NEEDS_OPACITY: bool;

	/// # Safety
	///
	/// If [`IntoPaths::NEEDS_OPACITY`] is `true`, the value passed to the `opacity`
	/// parameter must be initialised.
	unsafe fn into_paths(
		self,
		opacity: MaybeUninit<Signal<f32>>,
		#[cfg(debug_assertions)]
		is_set: bool,
		#[cfg(debug_assertions)]
		type_combination: &'static str
	) -> impl IntoView;

	// make NEEDS_OPACITY available from a value
	#[inline(always)]
	fn needs_opacity(&self) -> bool { Self::NEEDS_OPACITY }
}

unsafe impl IntoPaths for Path {
	// we dont use it
	const NEEDS_OPACITY: bool = false;

	#[inline(always)]
	unsafe fn into_paths(
		self,
		_: MaybeUninit<Signal<f32>>,
		#[cfg(debug_assertions)]
		_: bool,
		#[cfg(debug_assertions)]
		_: &'static str
	) -> impl IntoView {
		let Self { d } = self;

		view! {
			<path d=d />
		}
	}
}

unsafe impl IntoPaths for PathOpacity {
	const NEEDS_OPACITY: bool = true;

	#[inline(always)]
	unsafe fn into_paths(
		self,
		opacity: MaybeUninit<Signal<f32>>,
		#[cfg(debug_assertions)]
		is_set: bool,
		#[cfg(debug_assertions)]
		type_combination: &'static str
	) -> impl IntoView {
		let Self { d } = self;
		// SAFETY: caller initialised the variable according to NEEDS_OPACITY
		// which we set to true
		let opacity = unsafe { *opacity.assume_init_ref() };

		#[cfg(debug_assertions)]
		if !is_set {
			panic!("icon {type_combination} needs opacity but it wasn't set");
		}

		view! {
			<path d=d opacity=opacity />
		}
	}
}

macro_rules! gen_into_path {
	{ $($p:ident)* } => {
		unsafe impl<$($p),*> IntoPaths for ($($p),*)
		where
			$($p: IntoPaths),*
		{
			// if any of the type params read it, we indirectly read it and
			// need to declare this true
			const NEEDS_OPACITY: bool = false $(|| $p::NEEDS_OPACITY)*;

			#[inline(always)]
			unsafe fn into_paths(
				self,
				opacity: MaybeUninit<Signal<f32>>,
				#[cfg(debug_assertions)]
				is_set: bool,
				#[cfg(debug_assertions)]
				type_combination: &'static str
			) -> impl IntoView {
				#[expect(non_snake_case, reason = "macro")]
				let ($($p),*) = self;

				// SAFETY:
				// - caller set `opacity`
				// - `NEEDS_OPACITY` is set depending on type parameters,
				// - `into_paths` implementations of type parameters promises to behave too
				unsafe {
					($(
						$p.into_paths(
							opacity,
							#[cfg(debug_assertions)]
							is_set,
							#[cfg(debug_assertions)]
							type_combination
						)
					),*)
				}
			}
		}
	}
}

gen_into_path! { P1 P2 }
gen_into_path! { P1 P2 P3 }
gen_into_path! { P1 P2 P3 P4 }
gen_into_path! { P1 P2 P3 P4 P5 }
