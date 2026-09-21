use super::util::{ Icon, IconWeightData, Weight };

pub struct Bold;
impl Weight for Bold {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::bold_data()
	}

	#[inline]
	fn default() -> Self { Bold }
}

pub struct Duotone;
impl Weight for Duotone {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::duotone_data()
	}

	#[inline]
	fn default() -> Self { Duotone }
}

pub struct Fill;
impl Weight for Fill {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::fill_data()
	}

	#[inline]
	fn default() -> Self { Fill }
}

pub struct Light;
impl Weight for Light {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::light_data()
	}

	#[inline]
	fn default() -> Self { Light }
}

pub struct Regular;
impl Weight for Regular {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::regular_data()
	}

	#[inline]
	fn default() -> Self { Regular }
}

pub struct Thin;
impl Weight for Thin {
	#[inline]
	fn get_icon_data<I: Icon>() -> impl IconWeightData {
		I::thin_data()
	}

	#[inline]
	fn default() -> Self { Thin }
}
