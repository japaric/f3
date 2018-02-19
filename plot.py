# Plots sensor data logged using the `log-sensors` example
#
# Usage:
#
# $ pipenv run plot.py /path/to/data.txt $WHAT
#
# where `$WHAT` can be one of "accel", "gyro", "gyro-calibrated", "mag" or
# "mag-calibrated"

import cobs.cobs
import itertools
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import struct
import sys

# apply plot style
sns.set()

# Constants
N = 18  # frame size
K_AR = 8.75e-3  # gyroscope sensitivity
K_G = 2 / (1 << 15)  # accelerometer sensitivity
DT = 1 / 220  # sampling period

# Parse input file
with open(sys.argv[1], 'rb') as f:
    data = f.read()

mx, my, mz = [], [], []
arx, ary, arz = [], [], []
gx, gy, gz = [], [], []

for (is_separator, frame) in itertools.groupby(data, lambda x: x == 0):
    if is_separator:
        continue

    try:
        frame = cobs.cobs.decode(bytes(frame))
    except cobs.cobs.DecodeError:
        sys.stderr.write('X')
        sys.stderr.flush()
        continue

    if len(frame) != N:
        sys.stderr.write('!')
        sys.stderr.flush()
        continue

    start = 0
    mx.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2
    my.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2
    mz.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2

    arx.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2
    ary.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2
    arz.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2

    gx.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2
    gy.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2
    gz.append(struct.unpack('<h', frame[start:start+2])[0])
    start += 2

    assert(start == N)

target = sys.argv[2]

# Scale data
mx = np.array(mx)
my = np.array(my)
mz = np.array(mz)

mx_max = max(mx)
mx_min = min(mx)
my_max = max(my)
my_min = min(my)
mz_max = max(mz)
mz_min = min(mz)

mx_bias = (mx_max + mx_min) / 2
my_bias = (my_max + my_min) / 2
mz_bias = (mz_max + mz_min) / 2

mx_range = (mx_max - mx_min) / 2
my_range = (my_max - my_min) / 2
mz_range = (mz_max - mz_min) / 2

if target == 'mag-calibrated':
    mx = (mx - mx_bias) / mx_range
    my = (my - my_bias) / my_range
    mz = (mz - mz_bias) / mz_range

    mx_max = (mx_max - mx_bias) / mx_range
    mx_min = (mx_min - mx_bias) / mx_range
    my_max = (my_max - my_bias) / my_range
    my_min = (my_min - my_bias) / my_range
    mz_max = (mz_max - mz_bias) / mz_range
    mz_min = (mz_min - mz_bias) / mz_range

mxy = max([abs(mx_max), abs(mx_min), abs(my_max), abs(my_min)])
myz = max([abs(my_max), abs(my_min), abs(mz_max), abs(mz_min)])
mxz = max([abs(mx_max), abs(mx_min), abs(mz_max), abs(mz_min)])

arx_mean = np.mean(arx) * K_AR
ary_mean = np.mean(ary) * K_AR
arz_mean = np.mean(arz) * K_AR

if target == 'gyro-calibrated':
    arx = np.array(arx) * K_AR - arx_mean
    ary = np.array(ary) * K_AR - ary_mean
    arz = np.array(arz) * K_AR - arz_mean

    arx_mean = 0
    ary_mean = 0
    arz_mean = 0
else:
    arx = np.array(arx) * K_AR
    ary = np.array(ary) * K_AR
    arz = np.array(arz) * K_AR

gx = np.array(gx) * K_G
gy = np.array(gy) * K_G
gz = np.array(gz) * K_G

gx_mean = np.mean(gx)
gy_mean = np.mean(gy)
gz_mean = np.mean(gz)

# Plot
x = np.arange(0, len(arx)) * DT
if target == 'accel':
    plt.subplot(221)
    plt.plot(x, gx)
    plt.plot([x[0], x[-1]], np.ones(2) * gx_mean, label='mean')
    plt.xlim(round(x[0]), round(x[-1]))
    plt.legend()
    plt.ylabel('Acceleration (g)')
    plt.title(r'$G_x$')

    plt.subplot(222)
    plt.plot(x, gy)
    plt.plot([x[0], x[-1]], np.ones(2) * gy_mean, label='mean')
    plt.xlim(round(x[0]), round(x[-1]))
    plt.legend()
    plt.title(r'$G_y$')

    plt.subplot(223)
    plt.plot(x, gz)
    plt.plot([x[0], x[-1]], np.ones(2) * gz_mean, label='mean')
    plt.xlim(round(x[0]), round(x[-1]))
    plt.legend()
    plt.xlabel('Time (s)')
    plt.ylabel('Acceleration (g)')
    plt.title(r'$G_z$')

    g = np.sqrt(gx**2 + gy**2 + gz**2)
    plt.subplot(224)
    plt.plot(x, g)
    plt.xlim(round(x[0]), round(x[-1]))
    plt.xlabel('Time (s)')
    plt.title(r'$\|G\|$')

    plt.suptitle('Accelerometer data')
    plt.tight_layout()
    plt.subplots_adjust(top=0.88)

    plt.savefig(target + '.svg')
    plt.close()

if target == 'gyro' or target == 'gyro-calibrated':
    plt.subplot(221)
    plt.plot(x, arx)
    plt.plot([x[0], x[-1]], np.ones(2) * arx_mean, label='mean')
    plt.xlim(round(x[0]), round(x[-1]))
    plt.legend()
    plt.ylabel('Angular rate (dps)')
    plt.title(r'$AR_x$')

    plt.subplot(222)
    plt.plot(x, ary)
    plt.plot([x[0], x[-1]], np.ones(2) * ary_mean, label='mean')
    plt.xlim(round(x[0]), round(x[-1]))
    plt.legend()
    plt.title(r'$AR_y$')

    plt.subplot(223)
    plt.plot(x, arz)
    plt.plot([x[0], x[-1]], np.ones(2) * arz_mean, label='mean')
    plt.xlim(round(x[0]), round(x[-1]))
    plt.legend()
    plt.xlabel('Time (s)')
    plt.ylabel('Angular rate (dps)')
    plt.title(r'$AR_z$')

    ar = np.sqrt(arx**2 + ary**2 + arz**2)
    plt.subplot(224)
    plt.plot(x, ar)
    plt.xlim(round(x[0]), round(x[-1]))
    plt.xlabel('Time (s)')
    plt.title(r'$\|AR\|$')

    if target == 'gyro-calibrated':
        plt.suptitle('Calibrated gyroscope data')
    else:
        plt.suptitle('Gyroscope data')

    plt.tight_layout()
    plt.subplots_adjust(top=0.88)

    plt.savefig(target + '.svg')
    plt.close()

if target == 'mag' or target == 'mag-calibrated':
    ax = plt.subplot(221)
    plt.plot(mx, my, ',')
    plt.xlim(-mxy, mxy)
    plt.ylim(-mxy, mxy)
    ax.set_aspect(1)
    plt.xlabel(r'$M_X$')
    plt.ylabel(r'$M_Y$')
    plt.title(r'$M_{XY}$')

    ax = plt.subplot(222)
    plt.plot(my, mz, ',')
    plt.xlim(-myz, myz)
    plt.ylim(-myz, myz)
    ax.set_aspect(1)
    plt.xlabel(r'$M_Y$')
    plt.ylabel(r'$M_Z$')
    plt.title(r'$M_{YZ}$')

    ax = plt.subplot(223)
    plt.plot(mx, mz, ',')
    plt.xlim(-mxz, mxz)
    plt.ylim(-mxz, mxz)
    ax.set_aspect(1)
    plt.xlabel(r'$M_X$')
    plt.ylabel(r'$M_Z$')
    plt.title(r'$M_{XZ}$')

    m = np.sqrt(mx**2 + my**2 + mz**2)
    plt.subplot(224)
    plt.plot(x, m)
    plt.xlim(round(x[0]), round(x[-1]))
    plt.xlabel('Time (s)')
    plt.title(r'$\|M\|$')

    if target == 'mag-calibrated':
        plt.suptitle('Calibrated magnetometer data')
    else:
        plt.suptitle('Magnetometer data')

    plt.tight_layout()
    plt.subplots_adjust(top=0.88)

    plt.savefig(target + '.svg')
    plt.close()

    if target == 'mag-calibrated':
        print()
        print('X(bias =', mx_bias, ', range =', mx_range, ')')
        print('Y(bias =', my_bias, ', range =', my_range, ')')
        print('Z(bias =', mz_bias, ', range =', mz_range, ')')
