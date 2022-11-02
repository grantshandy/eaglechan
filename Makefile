all: css server

css:
	npx tailwindcss -i ./src/css/input.css -o ./src/css/style.css
	
server:
	cargo build --release
	
run: css
	cargo run

clean:
	cargo clean
	rm ./src/css/style.css