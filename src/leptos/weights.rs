use super::util::{ Icon, IntoPaths, Weight };

pub struct Bold;
impl Weight for Bold {
	const INIT: Bold = Bold;

	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::bold_data()
	}
}

pub struct Duotone;
impl Weight for Duotone {
	const INIT: Duotone = Duotone;

	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::duotone_data()
	}
}

pub struct Fill;
impl Weight for Fill {
	const INIT: Fill = Fill;

	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::fill_data()
	}
}

pub struct Light;
impl Weight for Light {
	const INIT: Light = Light;

	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::light_data()
	}
}

pub struct Regular;
impl Weight for Regular {
	const INIT: Regular = Regular;

	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::regular_data()
	}
}

pub struct Thin;
impl Weight for Thin {
	const INIT: Thin = Thin;

	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::thin_data()
	}
}
