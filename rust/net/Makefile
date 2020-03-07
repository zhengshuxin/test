all: server tcp-server udp_server udp_client
clean:
	rm -f server tcp-server udp_server udp_client

server: server.rs
	rustc server.rs
tcp-server: tcp-server.rs
	rustc tcp-server.rs
udp_server: udp_server.rs
	rustc udp_server.rs
udp_client: udp_client.rs
	rustc udp_client.rs
