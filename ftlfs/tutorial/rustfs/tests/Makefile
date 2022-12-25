mount:
	RUST_LOG=trace cargo run -- --device=/home/vpt/ddriver -f -d  -s ./mnt
unmount:
	fusermount -u ./mnt
mount_mt:
	RUST_LOG=trace cargo run -- --device=/home/vpt/ddriver -f -d ./mnt
unit_test_debug:
	RUST_LOG=trace cargo test -- --nocapture --test-threads=1 --color=always
unit_test:
	RUST_LOG=error cargo test -- --test-threads=1 --color=always
ddriver_test:
	RUST_LOG=trace cargo test -- --nocapture --test-threads=1 --color=always --test ddriver::test
replacer_test:
	RUST_LOG=trace cargo test -- --nocapture --test-threads=1 --color=always --test buffer::replacer::test
buffer_test:
	cargo test -- --nocapture --test-threads=1 --color=always --test buffer::test
loop_buffer_test:
	number=1 ; while [[ $$number -le 100 ]] ; do \
            echo "test" $$number "start" ; \
            cargo test -- --nocapture --test-threads=1 --color=always --test buffer::test ; \
            ((number = number + 1)) ; \
        done
fs_test:
	RUST_LOG=trace cargo test -- --nocapture --test-threads=1 --color=always --test fs::test
clean:
	rm /home/vpt/ddriver