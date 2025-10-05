rm -rf .kronicler_data

python3 ./large_average.py

echo "Finished running large_average.\n\n"

rm -rf .kronicler_data

python3 ./large_average_no_in_mem_comp.py

echo "Finished running large_average_no_in_mem_comp.\n\n"

echo "Note: large_average has a higher insert time because it needs to store ground truth values in memory. The ground truth average will also be quicker because it uses only in-memory values."
