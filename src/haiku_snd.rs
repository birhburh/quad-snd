use {
    crate::{
        PlaySoundParams,
        mixer::{self, Mixer}
    },
};

pub use crate::mixer::Playback;

mod consts {
    pub const RATE: f32 = 44100.;
    pub const CHANNELS: u32 = 2;
    pub const BUFFER_SIZE: usize = 4096;
    pub const BUFFER_PARTS: usize = 4;
}

pub struct Sound {
    sound_id: u32,
}

pub struct AudioContext {
    pub(crate) mixer_ctrl: crate::mixer::MixerControl,
}
 
#[repr(C)]
pub struct BPushGameSound {
    _private: [u8; 0],
}

#[link(name = "shims_lib")]
extern "C" {
    fn push_game_sound_create_start(frame_rate: f32, channel_count: u32, framesPerBufferPart: libc::size_t, bufferPartCount: libc::size_t) -> *mut BPushGameSound;
    fn lock_next_page(pushGameSound: *mut BPushGameSound, outPagePtr: *const *mut libc::c_void, out_page_size: *mut libc::size_t);
    fn unlock_page(pushGameSound: *mut BPushGameSound, inPagePtr: *mut libc::c_void);
    fn push_game_sound_stop(pushGameSound: *mut BPushGameSound);
}

unsafe fn audio_thread(mut mixer: crate::mixer::Mixer) {
    let push_game_sound = push_game_sound_create_start(consts::RATE, consts::CHANNELS, consts::BUFFER_SIZE / consts::BUFFER_PARTS / consts::CHANNELS as usize / 2, consts::BUFFER_PARTS);
    
    loop {
        let mut page_buffer = std::ptr::null_mut();
        let mut buffer_size = 0;
        lock_next_page(push_game_sound, &mut page_buffer, &mut buffer_size);
        let buffer = std::slice::from_raw_parts_mut(page_buffer as *mut f32, buffer_size);
        mixer.fill_audio_buffer(buffer, buffer_size);
        unlock_page(push_game_sound, page_buffer);
    }
    push_game_sound_stop(push_game_sound);
}

impl AudioContext {
    pub fn new() -> AudioContext {
        let (mixer_builder, mixer_ctrl) = Mixer::new();
        let mixer = Box::new(mixer_builder.build());
        
        let (mixer_builder, mixer_ctrl) = Mixer::new();
        std::thread::spawn(move || unsafe {
            audio_thread(mixer_builder.build());
        });
        AudioContext { mixer_ctrl }
    }
}

impl Sound {
    pub fn load(ctx: &AudioContext, data: &[u8]) -> Sound {
        let sound_id = ctx.mixer_ctrl.load(data);

        Sound { sound_id }
    }

    pub fn play(&self, ctx: &AudioContext, params: PlaySoundParams) -> Playback {
        ctx.mixer_ctrl.play(self.sound_id, params)
    }

    pub fn stop(&self, ctx: &AudioContext) {
        ctx.mixer_ctrl.stop_all(self.sound_id);
    }

    pub fn set_volume(&self, ctx: &AudioContext, volume: f32) {
        ctx.mixer_ctrl.set_volume_all(self.sound_id, volume);
    }

    pub fn delete(&self, ctx: &AudioContext) {
        ctx.mixer_ctrl.delete(self.sound_id);
    }
}

