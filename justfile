test: 
	cargo test

# command to run daily development. Runs tests and if that passes,
# runs the day
dev day:
	cargo watch -x "test --bin {{day}}" -x "run --bin {{day}}"
