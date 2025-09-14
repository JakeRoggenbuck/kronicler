import json
import glob
import numpy as np
from scipy import stats
import pandas as pd
from pathlib import Path

def load_and_filter_data(filename, test_types=['test_sqlite', 'test_columnar', 'avg_sqlite', 'avg_columnar']):
    """Load JSON data and filter for specified test types, converting nanoseconds to milliseconds"""
    try:
        with open(filename, 'r') as f:
            data = json.load(f)

        # Filter data for the specified test types and convert ns to ms
        filtered_data = {}
        for test_type in test_types:
            # Convert nanoseconds to milliseconds (divide by 1,000,000)
            filtered_data[test_type] = [item[1] / 1_000_000 for item in data if item[0] == test_type]

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
            'label': label,
            'count': 0,
            'mean': np.nan,
            'median': np.nan,
            'mean_no_outliers': np.nan,
            'std': np.nan,
            'min': np.nan,
            'max': np.nan
        }

    data = np.array(data)
    data_no_outliers = remove_outliers_iqr(data)

    return {
        'label': label,
        'count': len(data),
        'mean': np.mean(data),
        'median': np.median(data),
        'mean_no_outliers': np.mean(data_no_outliers) if len(data_no_outliers) > 0 else np.nan,
        'std': np.std(data, ddof=1) if len(data) > 1 else np.nan,
        'min': np.min(data),
        'max': np.max(data),
        'outliers_removed': len(data) - len(data_no_outliers)
    }

def perform_t_test(data1, data2, label1, label2):
    """Perform two-sample t-test"""
    if len(data1) == 0 or len(data2) == 0:
        return {
            'comparison': f"{label1} vs {label2}",
            'statistic': np.nan,
            'p_value': np.nan,
            'significant': False,
            'note': 'Insufficient data for t-test'
        }

    try:
        # Perform two-sample t-test assuming unequal variances (Welch's t-test)
        statistic, p_value = stats.ttest_ind(data1, data2, equal_var=False)

        return {
            'comparison': f"{label1} vs {label2}",
            'statistic': statistic,
            'p_value': p_value,
            'significant': p_value < 0.05,
            'effect_size': (np.mean(data1) - np.mean(data2)) / np.sqrt((np.var(data1) + np.var(data2)) / 2)
        }
    except Exception as e:
        return {
            'comparison': f"{label1} vs {label2}",
            'statistic': np.nan,
            'p_value': np.nan,
            'significant': False,
            'note': f'Error in t-test: {e}'
        }

def calculate_speedup(sqlite_data, columnar_data, operation_name, mode_name):
    """Calculate speedup from SQLite to Columnar (SQLite as baseline)"""
    if len(sqlite_data) == 0 or len(columnar_data) == 0:
        return None

    sqlite_mean = np.mean(sqlite_data)
    columnar_mean = np.mean(columnar_data)
    sqlite_median = np.median(sqlite_data)
    columnar_median = np.median(columnar_data)

    # Calculate speedup (SQLite time / Columnar time)
    speedup_mean = sqlite_mean / columnar_mean
    speedup_median = sqlite_median / columnar_median

    # Calculate speedup without outliers
    sqlite_no_outliers = remove_outliers_iqr(sqlite_data)
    columnar_no_outliers = remove_outliers_iqr(columnar_data)

    speedup_no_outliers = np.nan
    if len(sqlite_no_outliers) > 0 and len(columnar_no_outliers) > 0:
        speedup_no_outliers = np.mean(sqlite_no_outliers) / np.mean(columnar_no_outliers)

    return {
        'operation': operation_name,
        'mode': mode_name,
        'sqlite_mean_ms': sqlite_mean,
        'columnar_mean_ms': columnar_mean,
        'sqlite_median_ms': sqlite_median,
        'columnar_median_ms': columnar_median,
        'speedup_mean': speedup_mean,
        'speedup_median': speedup_median,
        'speedup_no_outliers': speedup_no_outliers,
        'time_saved_ms': sqlite_mean - columnar_mean,
        'percent_faster': ((speedup_mean - 1) * 100) if speedup_mean > 0 else 0
    }

def analyze_mode_data(mode_data, mode_name):
    """Analyze data for a specific mode (sync or concurrent)"""
    print(f"\n{'='*50}")
    print(f"=== {mode_name.upper()} MODE ANALYSIS ===")
    print(f"{'='*50}")

    # Initialize results
    results = []
    speedup_results = []

    # Calculate statistics for each test type
    for test_type, data in mode_data.items():
        if len(data) > 0:
            stats_result = calculate_statistics(data, f"{mode_name}_{test_type}")
            results.append(stats_result)
            print(f"\n--- {test_type.upper()} ({mode_name}) ---")
            print(f"Count: {stats_result['count']:,}")
            print(f"Mean: {stats_result['mean']:,.3f} ms")
            print(f"Median: {stats_result['median']:,.3f} ms")
            print(f"Mean (no outliers): {stats_result['mean_no_outliers']:,.3f} ms")
            print(f"Std Dev: {stats_result['std']:,.3f} ms")
            print(f"Min: {stats_result['min']:,.3f} ms")
            print(f"Max: {stats_result['max']:,.3f} ms")
            print(f"Outliers removed: {stats_result['outliers_removed']}")

    # Calculate and display speedup analysis
    print(f"\n--- SPEEDUP ANALYSIS FOR {mode_name.upper()} MODE (SQLite as Baseline) ---")

    # Insert operations speedup
    if len(mode_data['test_sqlite']) > 0 and len(mode_data['test_columnar']) > 0:
        speedup = calculate_speedup(mode_data['test_sqlite'], mode_data['test_columnar'],
                                   'Insert', mode_name)
        if speedup:
            speedup_results.append(speedup)
            print(f"\nInsert Operations ({mode_name}):")
            print(f"  SQLite mean: {speedup['sqlite_mean_ms']:,.3f} ms")
            print(f"  Columnar mean: {speedup['columnar_mean_ms']:,.3f} ms")
            print(f"  Speedup (mean): {speedup['speedup_mean']:.2f}x faster")
            print(f"  Speedup (median): {speedup['speedup_median']:.2f}x faster")
            if not np.isnan(speedup['speedup_no_outliers']):
                print(f"  Speedup (no outliers): {speedup['speedup_no_outliers']:.2f}x faster")
            print(f"  Time saved: {speedup['time_saved_ms']:,.3f} ms per operation")
            print(f"  Columnar is {speedup['percent_faster']:.1f}% faster")

    # Average operations speedup
    if len(mode_data['avg_sqlite']) > 0 and len(mode_data['avg_columnar']) > 0:
        speedup = calculate_speedup(mode_data['avg_sqlite'], mode_data['avg_columnar'],
                                   'Average', mode_name)
        if speedup:
            speedup_results.append(speedup)
            print(f"\nAverage Operations ({mode_name}):")
            print(f"  SQLite mean: {speedup['sqlite_mean_ms']:,.3f} ms")
            print(f"  Columnar mean: {speedup['columnar_mean_ms']:,.3f} ms")
            print(f"  Speedup (mean): {speedup['speedup_mean']:.2f}x faster")
            print(f"  Speedup (median): {speedup['speedup_median']:.2f}x faster")
            if not np.isnan(speedup['speedup_no_outliers']):
                print(f"  Speedup (no outliers): {speedup['speedup_no_outliers']:.2f}x faster")
            print(f"  Time saved: {speedup['time_saved_ms']:,.3f} ms per operation")
            print(f"  Columnar is {speedup['percent_faster']:.1f}% faster")

    # Perform t-tests within this mode
    print(f"\n--- T-TEST RESULTS FOR {mode_name.upper()} MODE ---")

    # Test insert data: sqlite vs columnar
    if len(mode_data['test_sqlite']) > 0 and len(mode_data['test_columnar']) > 0:
        t_result = perform_t_test(mode_data['test_sqlite'], mode_data['test_columnar'],
                                 f'{mode_name} Insert SQLite', f'{mode_name} Insert Columnar')
        print(f"\nInsert Performance ({mode_name}): {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if 'effect_size' in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")

    # Test average data: sqlite vs columnar
    if len(mode_data['avg_sqlite']) > 0 and len(mode_data['avg_columnar']) > 0:
        t_result = perform_t_test(mode_data['avg_sqlite'], mode_data['avg_columnar'],
                                 f'{mode_name} Average SQLite', f'{mode_name} Average Columnar')
        print(f"\nAverage Performance ({mode_name}): {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if 'effect_size' in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")

    # Cross-category comparisons within mode
    if len(mode_data['test_sqlite']) > 0 and len(mode_data['avg_sqlite']) > 0:
        t_result = perform_t_test(mode_data['test_sqlite'], mode_data['avg_sqlite'],
                                 f'{mode_name} Insert SQLite', f'{mode_name} Average SQLite')
        print(f"\nSQLite ({mode_name}): {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if 'effect_size' in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")

    if len(mode_data['test_columnar']) > 0 and len(mode_data['avg_columnar']) > 0:
        t_result = perform_t_test(mode_data['test_columnar'], mode_data['avg_columnar'],
                                 f'{mode_name} Insert Columnar', f'{mode_name} Average Columnar')
        print(f"\nColumnar ({mode_name}): {t_result['comparison']}")
        print(f"  t-statistic: {t_result['statistic']:.4f}")
        print(f"  p-value: {t_result['p_value']:.6f}")
        print(f"  Significant (p < 0.05): {t_result['significant']}")
        if 'effect_size' in t_result:
            print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")

    return results, speedup_results

def cross_mode_comparisons(sync_data, concurrent_data):
    """Perform comparisons between sync and concurrent modes"""
    print(f"\n{'='*50}")
    print("=== CROSS-MODE COMPARISONS (SYNC vs CONCURRENT) ===")
    print(f"{'='*50}")

    test_types = ['test_sqlite', 'test_columnar', 'avg_sqlite', 'avg_columnar']

    for test_type in test_types:
        if len(sync_data[test_type]) > 0 and len(concurrent_data[test_type]) > 0:
            t_result = perform_t_test(sync_data[test_type], concurrent_data[test_type],
                                     f'Sync {test_type}', f'Concurrent {test_type}')
            print(f"\n{test_type.replace('_', ' ').title()}: {t_result['comparison']}")
            print(f"  t-statistic: {t_result['statistic']:.4f}")
            print(f"  p-value: {t_result['p_value']:.6f}")
            print(f"  Significant (p < 0.05): {t_result['significant']}")
            if 'effect_size' in t_result:
                print(f"  Effect size (Cohen's d): {t_result['effect_size']:.4f}")

            # Show means for context
            sync_mean = np.mean(sync_data[test_type])
            concurrent_mean = np.mean(concurrent_data[test_type])
            print(f"  Sync mean: {sync_mean:.3f} ms")
            print(f"  Concurrent mean: {concurrent_mean:.3f} ms")
            print(f"  Difference: {abs(sync_mean - concurrent_mean):.3f} ms")

def analyze_files():
    """Main analysis function"""
    # Find all relevant JSON files
    sync_insert_files = glob.glob('sync_insert_data.json')
    concurrent_insert_files = glob.glob('concurrent_insert_data.json')
    sync_avg_files = glob.glob('sync_avg_data.json')
    concurrent_avg_files = glob.glob('concurrent_avg_data.json')

    print("=== PERFORMANCE DATA ANALYSIS (Times in milliseconds) ===\n")
    print(f"Found sync insert files: {sync_insert_files}")
    print(f"Found concurrent insert files: {concurrent_insert_files}")
    print(f"Found sync average files: {sync_avg_files}")
    print(f"Found concurrent average files: {concurrent_avg_files}")

    # Initialize data structures for both modes
    sync_data = {
        'test_sqlite': [],
        'test_columnar': [],
        'avg_sqlite': [],
        'avg_columnar': []
    }

    concurrent_data = {
        'test_sqlite': [],
        'test_columnar': [],
        'avg_sqlite': [],
        'avg_columnar': []
    }

    # Process sync files
    for filename in sync_insert_files:
        print(f"Processing {filename}...")
        data = load_and_filter_data(filename, ['test_sqlite', 'test_columnar'])
        if data:
            for test_type in ['test_sqlite', 'test_columnar']:
                sync_data[test_type].extend(data.get(test_type, []))

    for filename in sync_avg_files:
        print(f"Processing {filename}...")
        data = load_and_filter_data(filename, ['avg_sqlite', 'avg_columnar'])
        if data:
            for test_type in ['avg_sqlite', 'avg_columnar']:
                sync_data[test_type].extend(data.get(test_type, []))

    # Process concurrent files
    for filename in concurrent_insert_files:
        print(f"Processing {filename}...")
        data = load_and_filter_data(filename, ['test_sqlite', 'test_columnar'])
        if data:
            for test_type in ['test_sqlite', 'test_columnar']:
                concurrent_data[test_type].extend(data.get(test_type, []))

    for filename in concurrent_avg_files:
        print(f"Processing {filename}...")
        data = load_and_filter_data(filename, ['avg_sqlite', 'avg_columnar'])
        if data:
            for test_type in ['avg_sqlite', 'avg_columnar']:
                concurrent_data[test_type].extend(data.get(test_type, []))

    # Analyze each mode separately
    sync_results, sync_speedup = analyze_mode_data(sync_data, "sync")
    concurrent_results, concurrent_speedup = analyze_mode_data(concurrent_data, "concurrent")

    # Perform cross-mode comparisons
    cross_mode_comparisons(sync_data, concurrent_data)

    # Display comprehensive speedup summary
    print(f"\n{'='*60}")
    print("=== COMPREHENSIVE SPEEDUP SUMMARY (SQLite as Baseline) ===")
    print(f"{'='*60}")

    all_speedups = sync_speedup + concurrent_speedup
    if all_speedups:
        speedup_df = pd.DataFrame(all_speedups)

        print("\nSpeedup Summary Table:")
        display_cols = ['operation', 'mode', 'speedup_mean', 'speedup_median', 'speedup_no_outliers', 'percent_faster']
        if not speedup_df.empty:
            print(speedup_df[display_cols].to_string(index=False, float_format='%.2f'))

        print("\nDetailed Performance Comparison:")
        for speedup in all_speedups:
            print(f"\n{speedup['operation']} Operations ({speedup['mode'].title()} Mode):")
            print(f"  • Columnar is {speedup['speedup_mean']:.2f}x faster than SQLite")
            print(f"  • Performance improvement: {speedup['percent_faster']:.1f}%")
            print(f"  • Time saved per operation: {speedup['time_saved_ms']:,.3f} ms")

            # Show which is better
            if speedup['speedup_mean'] > 1:
                print(f"  ✓ Columnar significantly outperforms SQLite")
            elif speedup['speedup_mean'] < 1:
                print(f"  ✗ SQLite actually performs better (Columnar is {1/speedup['speedup_mean']:.2f}x slower)")
            else:
                print(f"  ≈ Performance is roughly equivalent")

    # Create comprehensive summary DataFrame
    all_results = sync_results + concurrent_results
    if all_results:
        df = pd.DataFrame(all_results)
        print(f"\n{'='*50}")
        print("=== COMPREHENSIVE STATISTICS TABLE (milliseconds) ===")
        print(f"{'='*50}")
        print(df.to_string(index=False, float_format='%.3f'))

        # Save results to CSV
        df.to_csv('performance_analysis_results.csv', index=False)

        # Save speedup results to separate CSV
        if all_speedups:
            speedup_df.to_csv('speedup_analysis_results.csv', index=False)
            print(f"\nStatistics saved to 'performance_analysis_results.csv'")
            print(f"Speedup analysis saved to 'speedup_analysis_results.csv'")
        else:
            print(f"\nResults saved to 'performance_analysis_results.csv'")

    return sync_data, concurrent_data, all_results, all_speedups

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
