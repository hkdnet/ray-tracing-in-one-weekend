out.png: out.ppm
	pnmtopng out.ppm > out.png

out.ppm: src/main.rs src/lib.rs
	cargo run > out.ppm
