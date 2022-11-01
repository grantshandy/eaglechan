all: css server

css:
	npx tailwindcss -i ./src/css/input.css -o ./src/css/style.css
	
server:
	RUST_LOG=info DATABASE_URL=sqlite:data.db cargo build --release
	
run: css
	cargo run
