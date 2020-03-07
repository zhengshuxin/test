all: channel channel2 channel_s sync_channel mpsc
clean:
	rm -f channel channel2 channel_s sync_channel mpsc
channel: channel.rs
	rustc channel.rs
channel2: channel2.rs
	rustc channel2.rs
channel_s: channel_s.rs
	rustc channel_s.rs
sync_channel: sync_channel.rs
	rustc sync_channel.rs
mpsc: mpsc.rs
	rustc mpsc.rs
