all: css server

css:
	npx tailwindcss -i ./src/input.css -o ./src/styles.css
	
server:
	cargo build --release
	
run: css
	cargo run
