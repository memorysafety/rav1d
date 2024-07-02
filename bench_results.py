import re

def parse_time(time_str):
    return float(time_str)

def calculate_average(times):
    return sum(times) / len(times)

def main():
    rav1d_times = []
    dav1d_times = []
    current_decoder = None

    data = '''
testing dav1d

real	47.214
user	270.204236
sys	6.683360
testing rav1d

real	58.240
user	357.863738
sys	9.970720
testing dav1d

real	59.890
user	379.184440
sys	8.899621
testing rav1d

real	70.354
user	447.472465
sys	11.418514
testing dav1d

real	61.442
user	392.380113
sys	9.166131
testing rav1d

real	69.864
user	443.213629
sys	11.830889
testing dav1d

real	61.312
user	392.964597
sys	9.255956
testing rav1d

real	69.378
user	442.190257
sys	11.543815
testing dav1d

real	61.320
user	393.89938
sys	9.47970
testing rav1d

real	69.944
user	443.964194
sys	11.637191
testing dav1d

real	61.523
user	392.879454
sys	9.100157
testing rav1d

real	69.657
user	443.42958
sys	11.803730
testing dav1d

real	61.359
user	393.30041
sys	9.258827
testing rav1d

real	69.262
user	441.865210
sys	11.250262
testing dav1d

real	61.380
user	394.61714
sys	8.902229
testing rav1d

real	69.670
user	443.120249
sys	11.486521
testing dav1d

real	61.533
user	393.521276
sys	9.432644
testing rav1d

real	69.561
user	444.259118
sys	11.996127
testing dav1d

real	61.334
user	393.363427
sys	9.20679
testing rav1d

real	69.599
user	445.186791
sys	11.423909


'''

    for line in data.strip().split('\n'):
        if line.startswith('#'):
            continue
        elif line.startswith('real'):
            time = parse_time(line.split()[1])
            if current_decoder == 'rav1d':
                rav1d_times.append(time)
            elif current_decoder == 'dav1d':
                dav1d_times.append(time)
        elif line.startswith('testing'):
            current_decoder = line.split()[1]

    avg_rav1d = calculate_average(rav1d_times)
    avg_dav1d = calculate_average(dav1d_times)

    percentage_difference = ((avg_rav1d - avg_dav1d) / avg_dav1d) * 100

    print(f"Average time for rav1d: {avg_rav1d:.3f} seconds")
    print(f"Average time for dav1d: {avg_dav1d:.3f} seconds")
    print(f"rav1d is {percentage_difference:.2f}% slower than dav1d")

if __name__ == "__main__":
    main()