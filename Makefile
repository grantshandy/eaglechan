all: css
	DATABASE_URL=sqlite:src/template.db cargo build --release

css:
	npx tailwindcss -i ./src/css/input.css -o ./src/css/style.css --minify
	
run: css
	RUST_LOG=info DATABASE_URL=sqlite:src/template.db cargo run

clean:
	cargo clean
	rm ./src/css/style.css