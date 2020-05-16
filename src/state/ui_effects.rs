pub struct UiEffects {
    pub frame_around_current_path: bool,
    pub frame_around_space_freed: bool,
    pub current_path_is_red: bool,
    pub scanning_visual_indicator: bool,
}

impl UiEffects {
    pub fn new () -> Self {
        Self {
            frame_around_current_path: false,
            frame_around_space_freed: false,
            current_path_is_red: false,
            scanning_visual_indicator: false,
        }
    }
    pub fn toggle_scanning_visual_indicator(&mut self) {
        self.scanning_visual_indicator = !self.scanning_visual_indicator;
    }
}
