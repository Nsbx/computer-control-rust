use windows_volume_control::AudioController;

pub fn main(volume_val: f32) {
    unsafe {
        let mut controller = AudioController::init(None);

        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();

        let master_session = controller.get_session_by_name("master".to_string());

        master_session.unwrap().setVolume(volume_val);
    }
}