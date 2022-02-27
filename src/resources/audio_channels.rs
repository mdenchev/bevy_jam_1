use bevy_kira_audio::{Audio, AudioChannel};

#[derive(Debug, Copy, Clone)]
pub enum AudioChannelId {
    Music,
    Audio,
}

pub struct AudioChannels {
    pub music: AudioChannel,
    pub music_volume: f32,
    pub audio: AudioChannel,
    pub audio_volume: f32,
}

impl Default for AudioChannels {
    fn default() -> Self {
        Self {
            music: AudioChannel::new("music".to_owned()),
            music_volume: 1.0,
            audio: AudioChannel::new("audio".to_owned()),
            audio_volume: 1.0,
        }
    }
}

impl AudioChannels {
    pub fn set_volume(&mut self, audio: &Audio, id: AudioChannelId, volume: f32) {
        match id {
            AudioChannelId::Music => {
                self.music_volume = volume;
                audio.set_volume_in_channel(volume, &self.music);
            }
            AudioChannelId::Audio => {
                self.audio_volume = volume;
                audio.set_volume_in_channel(volume, &self.audio);
            }
        }
    }
}
