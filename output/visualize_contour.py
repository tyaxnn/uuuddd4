import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# データを読み込み（ヘッダー行を手動で指定）
column_names = ['energy', 'line_start_kx', 'line_start_ky', 'line_end_kx', 'line_end_ky', 'bcd_x', 'bcd_y', 'spin', 'band_index']
df = pd.read_csv('contour_lines.dat', comment='#', names=column_names)

# ユニークなスピンとバンドの組み合わせを取得
spins = df['spin'].unique()
bands = df['band_index'].unique()

# 図を作成
fig, axes = plt.subplots(len(spins), len(bands), figsize=(5*len(bands), 5*len(spins)))
if len(spins) == 1 and len(bands) == 1:
    axes = [axes]
elif len(spins) == 1 or len(bands) == 1:
    axes = axes.reshape(-1)
else:
    axes = axes.flatten()

plot_idx = 0
for spin in spins:
    for band in bands:
        # 該当するスピンとバンドのデータを抽出
        subset = df[(df['spin'] == spin) & (df['band_index'] == band)]
        
        if len(subset) == 0:
            continue
            
        ax = axes[plot_idx] if len(spins) * len(bands) > 1 else axes[0]
        
        # 各エネルギーレベルごとに異なる色で描画
        energies = subset['energy'].unique()
        colors = plt.cm.viridis(np.linspace(0, 1, len(energies)))
        
        for energy, color in zip(energies, colors):
            energy_data = subset[subset['energy'] == energy]
            
            # 線分を描画
            for _, row in energy_data.iterrows():
                ax.plot([row['line_start_kx'], row['line_end_kx']], 
                       [row['line_start_ky'], row['line_end_ky']], 
                       color=color, alpha=0.7, linewidth=0.8)
        
        ax.set_xlabel('kx')
        ax.set_ylabel('ky')
        ax.set_title(f'Spin {spin}, Band {band}')
        ax.grid(True, alpha=0.3)
        ax.set_aspect('equal')
        
        plot_idx += 1

plt.tight_layout()
plt.savefig('contour_visualization.png', dpi=300, bbox_inches='tight')
plt.show()

# エネルギー範囲の情報を表示
print(f"Energy range: {df['energy'].min():.3f} to {df['energy'].max():.3f}")
print(f"Number of energy levels: {len(df['energy'].unique())}")
print(f"Spins: {sorted(spins)}")
print(f"Bands: {sorted(bands)}")