import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import os
from scipy.interpolate import griddata # 線形補完のためにインポート

# --- ファイル名生成の部分は変更なし ---
lam = 0.30
mesh = 400
div = 307
threshold_berry = 12
main_mesh = 10

bcd_x = False

def create_filename_base_2(prefix, lambda_val, j_val, mesh_x, mesh_y, div, threshold_berry, main_mesh, dir="data_qmd"):
    lambda_str = str(lambda_val).replace('.', 'p')
    j_str = str(j_val).replace('.', 'p')
    filename = (
        f"./{dir}/data_{prefix}_lambda{lambda_str}_j{j_str}_mesh_x{mesh_x}_mesh_y{mesh_y}_div{div}_thresh10em{threshold_berry}_main_mesh{main_mesh}.dat"
    )
    return filename

file_list = []
j_values_for_loop = []
for i in range(50):
    j_val = float("{:.2f}".format(0.01 * i))
    lambda_val = "0p30" # 元のlam変数を使うように修正
    j_values_for_loop.append(j_val)
    file_list.append(create_filename_base_2("stable", lambda_val, j_val, mesh, mesh, div, threshold_berry, main_mesh, "compare_6_spinmodel"))

# --- データ収集部分は変更なし ---
n_points = []
j_points = []
z_points = []

n_axis_template = None
for filename in file_list:
    if os.path.exists(filename):
        df_template = pd.read_csv(filename, comment='#', header=None, names=["n"], usecols=[0])
        n_axis_template = df_template["n"].values
        break

if n_axis_template is None:
    print("エラー: プロットに使用できるデータファイルが一つも見つかりませんでした。")
    exit()

print("データファイルを読み込んでいます...")
for i, filename in enumerate(file_list):
    j_val = j_values_for_loop[i]
    if j_val == 0.0:
        print("j=0.0の特別処理: bcd_x=0としてデータを追加します。")
        current_n_values = n_axis_template
        current_z_values = np.zeros_like(current_n_values, dtype=float)
    else:
        try:
            df = pd.read_csv(filename, comment='#', header=None, names=["n", "bcd_x", "bcd_y", "qmd_x", "qmd_y"], usecols=[0, 3, 4, 5, 6])
            current_n_values = df["n"].values

            if bcd_x:
                current_z_values = df["bcd_x"].values # bcd_xのみを使用
            else:
                current_z_values = df["qmd_y"].values # qmd_yのみを使用
        except FileNotFoundError:
            print(f"警告: ファイルが見つかりません。スキップします: {filename}")
            continue
    for n_val, z_val in zip(current_n_values, current_z_values):
        n_points.append(n_val)
        j_points.append(j_val)
        z_points.append(z_val)

# --- プロット部分の修正 ---
print("データを補間してプロットしています...")

if not n_points:
    print("エラー: プロットできるデータがありませんでした。")
else:
    # 1. データをNumPy配列に変換
    n_array = np.array(n_points, dtype=float)
    j_array = np.array(j_points, dtype=float)
    z_array = np.array(z_points, dtype=float)

    # 2. 補間先の新しいグリッドを定義 (n軸は500点に)
    grid_n = np.linspace(n_array.min(), n_array.max(), 500)
    grid_j = np.unique(j_array)
    grid_N, grid_J = np.meshgrid(grid_n, grid_j)

    # 3. 元のまばらなデータを、新しいグリッド上に線形補間
    grid_Z = griddata((n_array, j_array), z_array, (grid_N, grid_J), method='linear')

    # 4. 補間後の格子状データをimshowで描画
    plt.figure(figsize=(10, 8))
    
    im = plt.imshow(grid_Z,
                    extent=[grid_n.min(), grid_n.max(), grid_j.min(), grid_j.max()],
                    origin='lower',
                    aspect='auto',
                    cmap='bwr',
                    vmin=-5, vmax=5) # カラーバーの範囲を指定

    # plt.xlabel('n')
    # plt.ylabel('j', fontname='Arial', fontsize=24)
    # plt.xlabel('n', fontname='Arial', fontsize=24)
    plt.tick_params(axis='both', which='major', labelsize=24)
    # plt.title('2D Color Map of bcd_x vs. n and j (Linearly Interpolated)')
    cbar = plt.colorbar(im)
    cbar.ax.tick_params(labelsize=24)

    # x軸とy軸の表示範囲を指定
    plt.xlim(0, 1)
    plt.ylim(0, 0.5)
    
    plt.tight_layout()
    if bcd_x:
        out_title = "bcd_x_interpolated"
    else:
        out_title = "qmd_y_interpolated"
    plt.savefig(f'{out_title}_plot.png')
    plt.show()