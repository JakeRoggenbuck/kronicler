import json
import glob
import numpy as np
from scipy import stats
import pandas as pd
from pathlib import Path


def load_and_filter_data(
    filename, test_types=["test_sqlite", "test_columnar", "avg_sqlite", "avg_columnar"]
):
    """Load JSON data and filter for specified test types"""
    try:
        with open(filename, "r") as f:
            data = json.load(f)

        # Filter data for the specified test types
        filtered_data = {}
        for test_type in test_types:
            filtered_data[test_type] = [
                item[1] for item in data if item[0] == test_type
            ]

        return filtered_data
    except Exception as e:
        print(f"Error loading {filename}: {e}")
        return None


def remove_outliers_iqr(data, k=1.5):
    """Remove outliers using IQR method"""
    if len(data) == 0:
        return data

    data = np.array(data)
    q1 = np.percentile(data, 25)
    q3 = np.percentile(data, 75)
    iqr = q3 - q1
    lower_bound = q1 - k * iqr
    upper_bound = q3 + k * iqr

    return data[(data >= lower_bound) & (data <= upper_bound)]


def calculate_statistics(data, label):
    """Calculate all requested statistics for a dataset"""
    if len(data) == 0:
        return {
            "label": label,
            "count": 0,
            "mean": np.nan,
            "median": np.nan,
            "mean_no_outliers": np.nan,
            "std": np.nan,
            "min": np.nan,
            "max": np.nan,
        }

    data = np.array(data)
    data_no_outliers = remove_outliers_iqr(data)

    return {
        "label": label,
        "count": len(data),
        "mean": np.mean(data),
        "median": np.median(data),
        "mean_no_outliers": np.mean(data_no_outliers)
        if len(data_no_outliers) > 0
        else np.nan,
        "std": np.std(data, ddof=1) if len(data) > 1 else np.nan,
        "min": np.min(data),
        "max": np.max(data),
        "outliers_removed": len(data) - len(data_no_outliers),
    }


def perform_t_test(data1, data2, label1, label2):
    """Perform two-sample t-test"""
    if len(data1) == 0 or len(data2) == 0:
        return {
            "comparison": f"{label1} vs {label2}",
            "statistic": np.nan,
            "p_value": np.nan,
            "significant": False,
            "note": "Insufficient data for t-test",
        }

    try:
        # Perform two-sample t-test assuming unequal variances (Welch's t-test)
        statistic, p_value = stats.ttest_ind(data1, data2, equal_var=False)

        return {
            "comparison": f"{label1} vs {label2}",
            "statistic": statistic,
            "p_value": p_value,
            "significant": p_value < 0.05,
            "effect_size": (np.mean(data1) - np.mean(data2))
            / np.sqrt((np.var(data1) + np.var(data2)) / 2),
        }
    except Exception as e:
        return {
            "comparison": f"{label1} vs {label2}",
            "statistic": np.nan,
            "p_value": np.nan,
            "significant": False,
            "note": f"Error in t-test: {e}",
        }


def analyze_files():
    """Main analysis function"""
    # Find all relevant JSON files
    insert_files = glob.glob("*_insert_data.json")
    avg_files = glob.glob("*_avg_data.json")

    print("=== PERFORMANCE DATA ANALYSIS ===\n")
    print(f"Found {len(insert_files)} insert data files: {insert_files}")
    print(f"Found {len(avg_files)} average data files: {avg_files}")
    print()

    # Combine all data by test type
    all_data = {
        "test_sqlite": [],
        "test_columnar": [],
        "avg_sqlite": [],
        "avg_columnar": [],
    }

    # Process insert files
    for filename in insert_files:
        print(f"Processing {filename}...")
        data = load_and_filter_data(filename, ["test_sqlite", "test_columnar"])
        if data:
            for test_type in ["test_sqlite", "test_columnar"]:
                all_data[test_type].extend(data.get(test_type, []))

    # Process average files
    for filename in avg_files:
        print(f"Processing {filename}...")
        data = load_and_filter_data(filename, ["avg_sqlite", "avg_columnar"])
        if data:
            for test_type in ["avg_sqlite", "avg_columnar"]:
                all_data[test_type].extend(data.get(test_type, []))

    print()

    # Calculate statistics for each test type
    results = []
    for test_type, data in all_data.items():
        stats_result = calculate_statistics(data, test_type)
        results.append(stats_result)
        print(f"=== {test_type.upper()} STATISTICS ===")
        print(f"Count: {stats_result['count']:,}")
        print(f"Mean: {stats_result['mean']:,.2f}")
        print(f"Median: {stats_result['median']:,.2f}")
        print(f"Mean (no outliers): {stats_result['mean_no_outliers']:,.2f}")
        print(f"Std Dev: {stats_result['std']:,.2f}")
        print(f"Min: {stats_result['min']:,.2f}")
        print(f"Max: {stats_result['max']:,.2f}")
        print(f"Outliers removed: {stats_result['outliers_removed']}")
        print()

    # Perform t-tests
    print("=== T-TEST RESULTS ===")

    # Test insert data: sqlite vs columnar
    if len(all_data["test_sqlite"]) > 0 and len(all_data["test_columnar"]) > 0:
        t_result = perform_t_test(
            all_data["test_sqlite"],
            all_data["test_columnar"],
            "Insert SQLite",
            "Insert Columnar",
        )
        print(f"Insert Performance: {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if "effect_size" in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")
        print()

    # Test average data: sqlite vs columnar
    if len(all_data["avg_sqlite"]) > 0 and len(all_data["avg_columnar"]) > 0:
        t_result = perform_t_test(
            all_data["avg_sqlite"],
            all_data["avg_columnar"],
            "Average SQLite",
            "Average Columnar",
        )
        print(f"Average Performance: {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if "effect_size" in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")
        print()

    # Cross-category comparisons
    if len(all_data["test_sqlite"]) > 0 and len(all_data["avg_sqlite"]) > 0:
        t_result = perform_t_test(
            all_data["test_sqlite"],
            all_data["avg_sqlite"],
            "Insert SQLite",
            "Average SQLite",
        )
        print(f"SQLite: {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if "effect_size" in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")
        print()

    if len(all_data["test_columnar"]) > 0 and len(all_data["avg_columnar"]) > 0:
        t_result = perform_t_test(
            all_data["test_columnar"],
            all_data["avg_columnar"],
            "Insert Columnar",
            "Average Columnar",
        )
        print(f"Columnar: {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if "effect_size" in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")
        print()

    # Create summary DataFrame
    df = pd.DataFrame(results)
    print("=== SUMMARY TABLE ===")
    print(df.to_string(index=False, float_format="%.2f"))

    # Save results to CSV
    df.to_csv("performance_analysis_results.csv", index=False)
    print(f"\nResults saved to 'performance_analysis_results.csv'")

    return all_data, results


if __name__ == "__main__":
    # Check if required libraries are installed
    try:
        import scipy
        import pandas
    except ImportError as e:
        print(f"Missing required library: {e}")
        print("Please install with: pip install scipy pandas")
        exit(1)

    analyze_files()
