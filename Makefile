out.png: out.ppm
	pnmtopng out.ppm > out.png

out.ppm: main.rs
	cargo run > out.ppm
