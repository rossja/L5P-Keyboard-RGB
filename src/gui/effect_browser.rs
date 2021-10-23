use fltk::{
	browser::HoldBrowser,
	enums::{Color, FrameType},
	group::Tile,
	prelude::*,
};

const WHITE: u32 = 0xffffff;
const LIGHTER_GRAY: u32 = 0xcccccc;

pub struct EffectBrowser;

impl EffectBrowser {
	pub fn new(effects_list: &Vec<&str>) -> HoldBrowser {
		let mut effect_browser = HoldBrowser::new(0, 0, 310, 310, "").center_of_parent();
		for effect in effects_list.iter() {
			effect_browser.add(effect);
		}

		// Effect choice
		effect_browser.set_frame(FrameType::FlatBox);
		effect_browser.set_color(Color::from_u32(LIGHTER_GRAY));
		effect_browser.set_selection_color(Color::from_u32(WHITE));
		effect_browser.set_text_size(20);
		effect_browser.select(1);
		effect_browser
	}
}

pub struct EffectBrowserTile {
	pub effect_browser: HoldBrowser,
}

impl EffectBrowserTile {
	pub fn new(effects_list: &Vec<&str>) -> EffectBrowserTile {
		let mut effect_browser_tile = Tile::new(540, 0, 360, 360, "");
		let effect_browser = EffectBrowser::new(&effects_list);
		effect_browser_tile.end();

		effect_browser_tile.set_frame(FrameType::FlatBox);
		effect_browser_tile.set_color(Color::from_u32(0x222222));

		Self { effect_browser }
	}
}
