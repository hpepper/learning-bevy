all: assets/stick_figure.png
	cargo build

run: assets/stick_figure.png
	cargo run

assets/stick_figure.png: stick_figure.svg assets
	convert -background none $< $@

assets:
	mkdir assets

clean:
	- rm assets/stick_figure.png