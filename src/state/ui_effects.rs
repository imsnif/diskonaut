pub struct UiEffects {
    pub frame_around_current_path: bool,
    pub frame_around_space_freed: bool,
    pub current_path_is_red: bool,
    pub loading_progress_indicator: u64,
}

impl UiEffects {
    pub fn new () -> Self {
        Self {
            frame_around_current_path: false,
            frame_around_space_freed: false,
            current_path_is_red: false,
            loading_progress_indicator: 0,
        }
    }
    pub fn increment_loading_progress_indicator(&mut self) {
        // increasing and decreasing this number will increase
        // the scanning text animation speed
        self.loading_progress_indicator += 3;
    }
}
