import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# データを読み込み（ヘッダー行を手動で指定）
column_names = ['energy', 'line_start_kx', 'line_start_ky', 'line_end_kx', 'line_end_ky', 'bcd_x', 'bcd_y', 'spin', 'band_index']
df = pd.read_csv('contour_lines_uuuddd.dat', comment='#', names=column_names)



# -1.29に最も近いエネルギーを探す
target_energy = -1.11
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
plt.xlabel('kx', fontsize=24)
plt.ylabel('ky', fontsize=24)
plt.xlim(-2, 2)
plt.ylim(-2, 2)
# plt.title(f'Contours closest to E={closest_energy:.5f} (target={target_energy})')
plt.grid(True, alpha=0.3)
plt.gca().set_aspect('equal')
plt.tight_layout()
plt.savefig('contour_visualization_uuuddd.png', dpi=300, bbox_inches='tight')
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