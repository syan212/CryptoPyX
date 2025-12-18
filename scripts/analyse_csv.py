#!/usr/bin/env python3
import csv
from pathlib import Path


def _benchmark_matches(row: dict, benchmark_name: str) -> bool:
    return row['name'] == benchmark_name


file_path: Path = (
    Path(__file__).parent.parent
    / f'.benchmarks/{input("Enter benchmark file name (without.csv): ")}.csv'
)
field: str = input('Enter field name to analyse: ')
benchmark_name: str = input('Enter benchmark name to filter (leave blank for all): ')


with file_path.open(encoding='utf8') as f:
    reader = csv.DictReader(f)
    info = [
        float(row[field])
        for row in reader
        if (benchmark_name and _benchmark_matches(row, benchmark_name))
        or (not benchmark_name)
    ]


average: float = sum(info) / len(info) if info else 0.0
print(f'Average {field}: {average}')
