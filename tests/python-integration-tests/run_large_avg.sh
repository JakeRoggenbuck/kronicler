# Test: Does the average get the correct value?
#
# The main goal of this test is to double check that the average calculation is correct.
#
# ```
# Running insert took 76.3846685886383
# First row: Row(id=0, fields=["jake", Epoch(169), Epoch(367), Epoch(198)])
# Fetched all 100000 rows.
# Kronicler average:  200.19449000000037 ran in 2.1696090698242188e-05.
# Ground truth average:  200.19449 ran in 0.0007288455963134766.
# Finished running large_average.
#
#
# Running insert took 77.20267271995544
# First row: Row(id=0, fields=["jake", Epoch(191), Epoch(372), Epoch(181)])
# Fetched all 100000 rows.
# Kronicler average:  199.92816000000082 ran in 2.3126602172851562e-05.
# Finished running large_average_no_in_mem_comp.
# ```
#
# As you can see, in the first test the kronicler_data amortized const average 
# took `2.1696090698242188e-05` seconds but the ground truth written in Python 
# `0.0007288455963134766` seconds. The Rust amortized constant is faster.

rm -rf .kronicler_data

sleep 2

python3 ./large_average.py

echo -e "Finished running large_average.\n\n"

rm -rf .kronicler_data

sleep 2

python3 ./large_average_no_in_mem_comp.py

echo -e "Finished running large_average_no_in_mem_comp.\n\n"

echo "Note: large_average has a higher insert time because it needs to store ground truth values in memory. The ground truth average will also be quicker because it uses only in-memory values."
