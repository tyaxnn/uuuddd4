import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Use LaTeX-like fonts (Computer Modern via mathtext) without requiring external TeX
plt.rcParams['font.family'] = 'serif'
plt.rcParams['font.serif'] = ['Computer Modern Roman', 'DejaVu Serif']
plt.rcParams['mathtext.fontset'] = 'cm'
plt.rcParams['axes.labelsize'] = 24
plt.rcParams['xtick.labelsize'] = 16
plt.rcParams['ytick.labelsize'] = 16
plt.rcParams['legend.fontsize'] = 14

# model size for BZ (2 or 6)
size = 6
model=str = "uuudddtmd"

def draw_first_bz_on_ax(ax, size=6, color='k', linestyle='--'):
    """Draw 1st Brillouin zone hexagon on given axes.
    Uses the same formulas as src/consts.rs to compute kp depending on `size`.
    """
    SQRT_3 = 1.7320508075688772
    PI = np.pi
    TRI = 4.0 * PI / 9.0

    if size == 6:
        kp_x = TRI * SQRT_3 / 2.0
        kp_y = TRI * 0.5
    elif size == 2:
        base = 4.0 * SQRT_3 * PI / 9.0
        kp_x = base * 0.5
        kp_y = base * SQRT_3 * 0.5
    else:
        kp_x = TRI * SQRT_3 / 2.0
        kp_y = TRI * 0.5

    # create 6 vertices by rotating kp by multiples of 60 degrees
    angles = np.linspace(0, 2*np.pi, 7)  # includes endpoint to close
    verts = [(kp_x * np.cos(a) - kp_y * np.sin(a), kp_x * np.sin(a) + kp_y * np.cos(a)) for a in angles]
    xs = [v[0] for v in verts]
    ys = [v[1] for v in verts]
    ax.plot(xs, ys, linestyle=linestyle, color=color, linewidth=2)

# データを読み込み（ヘッダー行を手動で指定）
column_names = ['energy', 'line_start_kx', 'line_start_ky', 'line_end_kx', 'line_end_ky', 'bcd_x', 'bcd_y', 'spin', 'band_index']
df = pd.read_csv(f'contour_lines_{model}.dat', comment='#', names=column_names)



# -1.29に最も近いエネルギーを探す
target_energy = -1.15
df['energy_diff'] = (df['energy'] - target_energy).abs()
closest_energy = df.loc[df['energy_diff'].idxmin()]['energy']



# 全てのバンド・スピンの組み合わせを取得
band_spin_pairs = df[df['energy'] == closest_energy][['band_index', 'spin']].drop_duplicates()

# プロット
plt.figure(figsize=(6, 6))
for i, (_, row) in enumerate(band_spin_pairs.iterrows()):
    band = row['band_index']
    spin = row['spin']
    filtered = df[(df['energy'] == closest_energy) & (df['band_index'] == band) & (df['spin'] == spin)]
    color = 'red' if spin == 1 else 'blue' if spin == 0 else 'gray'
    for _, seg in filtered.iterrows():
        plt.plot([seg['line_start_kx'], seg['line_end_kx']],
                 [seg['line_start_ky'], seg['line_end_ky']],
                 color=color, alpha=0.8, linewidth=1.2,
                 label=f'Band {int(band)}, Spin {int(spin)}' if _ == filtered.index[0] else "")

# 凡例は一度だけ
handles, labels = plt.gca().get_legend_handles_labels()
by_label = dict(zip(labels, handles))
# plt.legend(by_label.values(), by_label.keys())
plt.xlabel(r'$k_x$', fontsize=24)
plt.ylabel(r'$k_y$', fontsize=24)
# set axis limits depending on model size
if size == 2:
    lim = 2.5
elif size == 6:
    lim = 1.5
else:
    lim = 2.5
plt.xlim(-lim, lim)
plt.ylim(-lim, lim)
# plt.title(f'Contours closest to E={closest_energy:.5f} (target={target_energy})')
# grid & aspect (no grid as requested)
plt.grid(False)
ax = plt.gca()
ax.set_aspect('equal')

# set integer ticks every 1 between -lim and lim (inclusive)
tick_start = int(np.ceil(-lim))
tick_end = int(np.floor(lim))
ticks = np.arange(tick_start, tick_end + 1, 1)
plt.xticks(ticks)
plt.yticks(ticks)

plt.tight_layout()

# draw first Brillouin zone (dashed hexagon)
draw_first_bz_on_ax(ax, size=size, color='k', linestyle='--')

plt.savefig(f'contour_visualization_{model}_{target_energy}.png', dpi=300, bbox_inches='tight')
plt.show()

# 情報表示
print(f"Closest energy to {target_energy}: {closest_energy}")
print("Band/Spin pairs:")
print(band_spin_pairs)
for i, (_, row) in enumerate(band_spin_pairs.iterrows()):
    band = row['band_index']
    spin = row['spin']
    count = len(df[(df['energy'] == closest_energy) & (df['band_index'] == band) & (df['spin'] == spin)])
    print(f"Band {int(band)}, Spin {int(spin)}: {count} segments")