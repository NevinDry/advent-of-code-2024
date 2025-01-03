run-all: 
	@for dir in $(shell ls -d day* | sort -V); do \
		echo "-- Running in $$dir --"; \
		start_time=$$(date +%s%3N); \
		( cd $$dir && cargo run --release --quiet); \
		end_time=$$(date +%s%3N); \
		elapsed_time=$$((end_time - start_time)); \
		echo "-- Runned in: $$elapsed_time milliseconds --"; \
		echo "-------------------------------------------"; \
	done