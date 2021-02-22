out.png: out.ppm
	pnmtopng out.ppm > out.png

out.ppm: src/main.rs
	cargo run > out.ppm
