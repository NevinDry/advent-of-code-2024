run-all: 
	@total_start_time=$$(date +%s); \
	total_elapsed_time=0; \
	for dir in $(shell ls -d day* | sort -V); do \
		echo "- Running in $$dir -"; \
		start_time=$$(date +%s%3N); \
		( cd $$dir && cargo run --release --quiet ); \
		end_time=$$(date +%s%3N); \
		elapsed_time=$$((end_time - start_time)); \
		echo "- Time taken: $$elapsed_time milliseconds -"; \
		total_elapsed_time=$$((total_elapsed_time + elapsed_time)); \
		echo "---------------------------------"; \
	done; \
	total_end_time=$$(date +%s); \
	total_time=$$((total_end_time - total_start_time)); \
	echo "-- Total time for all Puzzles: $$total_time seconds --";
