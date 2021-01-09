use crate::{
	audio_stream::{AudioStream, AudioStreamId},
	command::{sender::CommandSender, MixerCommand, StreamCommand},
	mixer::effect::{Effect, EffectHandle, EffectId, EffectSettings},
	AudioResult,
};

use super::TrackIndex;

pub struct TrackHandle {
	index: TrackIndex,
	command_sender: CommandSender,
}

impl TrackHandle {
	pub(crate) fn new(index: TrackIndex, command_sender: CommandSender) -> Self {
		Self {
			index,
			command_sender,
		}
	}

	pub fn index(&self) -> TrackIndex {
		self.index
	}

	pub fn add_effect(
		&mut self,
		effect: impl Effect + 'static,
		settings: EffectSettings,
	) -> AudioResult<EffectHandle> {
		let handle = EffectHandle::new(self.index, &settings, self.command_sender.clone());
		self.command_sender
			.push(MixerCommand::AddEffect(self.index, Box::new(effect), settings).into())?;
		Ok(handle)
	}

	pub fn remove_effect(&mut self, id: impl Into<EffectId>) -> AudioResult<()> {
		self.command_sender
			.push(MixerCommand::RemoveEffect(self.index, id.into()).into())
	}

	pub fn add_stream(&mut self, stream: impl AudioStream) -> AudioResult<AudioStreamId> {
		let stream_id = AudioStreamId::new();
		self.command_sender
			.push(StreamCommand::AddStream(stream_id, self.index(), Box::new(stream)).into())
			.map(|()| stream_id)
	}

	pub fn remove_stream(&mut self, id: AudioStreamId) -> AudioResult<()> {
		self.command_sender
			.push(StreamCommand::RemoveStream(id).into())
	}
}
