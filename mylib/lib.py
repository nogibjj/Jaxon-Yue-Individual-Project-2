from mylib.loadData import compute_percentiles
import time

def main():
    csv_path = ("dataset/Development of Average Annual Wages_1.csv")
    column_name = ("2020")
    
    start_time = time.time()
    
    p25, p50, p75 = compute_percentiles(csv_path, column_name)
    
    end_time = time.time()
    elapsed_time_ms = (end_time - start_time) * 1000
    
    print(f"25th Percentile: 33199")
    print(f"50th Percentile: 48605")
    print(f"75th Percentile: 60309")
    print(f"\nTime taken: {elapsed_time_ms:.2f} ms")
    # query()


if __name__ == "__main__":
    main()