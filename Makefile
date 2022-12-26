# Main commands
# ~~~~~~~~~~~~~

# Generates all tinantas supported by the program and writes them to stdout.
create_tinantas:
	cargo run --release --bin create_tinantas


# Generates all krdantas-prAtipadikas supported by the program and writes them to stdout.
create_krdantas:
	cargo run --release --bin create_krdantas


# Unit tests
# ~~~~~~~~~~

# Runs all unit tests in the crate.
integration_:
	cargo test --lib

# Generates a simple coverage report and writes it to disk as an HTML file.
coverage:
	cargo llvm-cov --html


# Integration tests
# ~~~~~~~~~~~~~~~~~

# Generates all tinantas supported by the program and writes them to disk.
create_test_files:
	cargo build --release
	./target/release/create_tinantas > test-files/tinantas-basic.csv
	# ./target/release/create_tinantas --sanadi Ric > test-files/tinantas-Ric.csv
	# ./target/release/create_tinantas --sanadi san > test-files/tinantas-san.csv
	./target/release/create_krdantas --krt ktvA > test-files/krdantas-ktvA.csv
	./target/release/create_krdantas --krt kta > test-files/krdantas-kta.csv

# Runs a full evaluation over all forms generated by vidyut-prakriya. `hash` is
# the SHA-256 hash of the test file. We use `hash` to verify test file
# integrity and ensure that our test cases are stable.
test_all:
	cargo build --release
	./target/release/test_tinantas \
		--test-cases test-files/tinantas-basic.csv \
	    --hash "08838cc8855cf62ab3102a757f90ce02c19dfc263bb7ece396420e76cc19f720"

# Work-in-progress test function for krdantas.
test_krt:
	cargo build --release
	./target/release/test_krdantas \
		--test-cases test-files/krdantas-ktvA.csv
	./target/release/test_krdantas \
		--test-cases test-files/krdantas-kta.csv

# Work-in-progress test function for subantas.
test_subantas:
	cargo build --release
	./target/release/test_subantas


# Performance
# ~~~~~~~~~~~

# Profiles the program's execution time on OSX. This command will probably not
# work on other operating systems.
profile-time-osx:
	cargo instruments -t time --release --bin create_test_file


# Other
# ~~~~~

# Generates project docs and opens them in your default browser.
docs:
	cargo doc --no-deps --open
