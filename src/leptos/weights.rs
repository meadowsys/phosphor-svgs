use super::util::{ Icon, IntoPaths, Weight };

pub struct Bold;
impl Weight for Bold {
	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::bold_data()
	}

	#[inline(always)]
	fn default() -> Self { Bold }
}

pub struct Duotone;
impl Weight for Duotone {
	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::duotone_data()
	}

	#[inline(always)]
	fn default() -> Self { Duotone }
}

pub struct Fill;
impl Weight for Fill {
	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::fill_data()
	}

	#[inline(always)]
	fn default() -> Self { Fill }
}

pub struct Light;
impl Weight for Light {
	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::light_data()
	}

	#[inline(always)]
	fn default() -> Self { Light }
}

pub struct Regular;
impl Weight for Regular {
	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::regular_data()
	}

	#[inline(always)]
	fn default() -> Self { Regular }
}

pub struct Thin;
impl Weight for Thin {
	#[inline(always)]
	fn get_icon_data<I: Icon>() -> impl IntoPaths {
		I::thin_data()
	}

	#[inline(always)]
	fn default() -> Self { Thin }
}
