all: assets/fonts/BevyTest.ttf
	cargo build

run: assets/fonts/BevyTest.ttf
	cargo run

assets/fonts/BevyTest.ttf: BevyTest.birdfont assets/fonts
	birdfont-export --output assets/fonts --ttf $<

assets/fonts:
	mkdir -p assets/fonts

clean:
	- rm assets/fonts/BevyTest.ttf
	- rm assets/fonts/BevyTestMac.ttf