import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.cm as cm
import matplotlib.patches as mpatches
import numpy as np
import os

def create_filename_base(prefix, lambda_val, j_val, mesh_x, mesh_y, div, threshold_berry):
    """
    設定値からファイル名のベース部分を生成する関数
    """
    # floatを文字列に変換し、'.'を'p'に置換
    lambda_str = str(lambda_val).replace('.', 'p')
    j_str = str(j_val).replace('.', 'p')

    # f-stringを使って最終的な文字列を組み立てる
    filename = (
        f"./data_qmd/data_{prefix}_lambda{lambda_str}_j{j_str}_mesh_x{mesh_x}_mesh_y{mesh_y}_div{div}_thresh10em{threshold_berry}.dat"
    )
    
    return filename

# ==== ファイル名リスト ====
file_list = [
    create_filename_base("One2Tmd", 0.3, 0.25, 2000, 2000, 307, 12),
    f"./data_from_uuuddd2/tmdone2_lambda0p3_j0p25_mu0_300_300_0p2_change_hamiltonian.dat"
]

labels = ["new", "old"]

row_num = 3

out_title = "compare_uuuddd2_uuuddd4_one2tmd"


# ==== 色設定 ====
num_files = len(file_list)
colors = cm.rainbow(np.linspace(0, 1, num_files))

# ==== matplotlib全体のフォントサイズ設定 ====
plt.rcParams['font.size'] = 24

# ==== プロット準備 ====
fig, axes = plt.subplots(nrows=row_num, ncols=1, figsize=(14, 16), sharex=True)  # 横幅を広げた

# ==== 全ファイルのy軸範囲をまとめて取得 ====
ymins = np.full(5, np.inf)
ymaxs = np.full(5, -np.inf)

for i, filename in enumerate(file_list):
    file_path = f"{filename}"
    df = pd.read_csv(file_path, comment='#', header=None,
                     names=["n","energy", "bc_sum", "bcd_x_sum", "bcd_y_sum", "qmd_x_sum", "qmd_y_sum"])
    
    color = colors[i]
    axes[0].plot(df["n"], df["bc_sum"], color=color)
    axes[1].plot(df["n"], df["bcd_x_sum"], color=color)
    axes[2].plot(df["n"], df["bcd_y_sum"], color=color)

    if row_num == 5 :
        axes[3].plot(df["n"], df["qmd_x_sum"], color=color)
        axes[4].plot(df["n"], df["qmd_y_sum"], color=color)

    # 最小最大更新
    ymins[0] = min(ymins[0], df["bc_sum"].min())
    ymins[1] = min(ymins[1], df["bcd_x_sum"].min())
    ymins[2] = min(ymins[2], df["bcd_y_sum"].min())
    if row_num == 5 :
        ymins[3] = min(ymins[3], df["qmd_x_sum"].min())
        ymins[4] = min(ymins[4], df["qmd_y_sum"].min())
    ymaxs[0] = max(ymaxs[0], df["bc_sum"].max())
    ymaxs[1] = max(ymaxs[1], df["bcd_x_sum"].max())
    ymaxs[2] = max(ymaxs[2], df["bcd_y_sum"].max())
    if row_num == 5 :
        ymaxs[3] = max(ymaxs[3], df["qmd_x_sum"].max())
        ymaxs[4] = max(ymaxs[4], df["qmd_y_sum"].max())

# ==== y軸範囲設定 ====
for ax, ymin, ymax in zip(axes, ymins, ymaxs):
    yrange = ymax - ymin
    if yrange < 2e-5:
        ax.set_ylim(-1e-5, 1e-5)
    elif yrange > 10:
        ax.set_ylim(-5, 5)
    else:
        ax.set_ylim(-abs(yrange/2), abs(yrange/2))

# ==== ラベルやタイトル ====
axes[0].set_ylabel("BC")
axes[1].set_ylabel("BCD X")
axes[2].set_ylabel("BCD Y")
if row_num == 5 :
    axes[3].set_ylabel("QMD X")
    axes[4].set_ylabel("QMD Y")
    axes[4].set_xlabel("n")
else:
    axes[2].set_xlabel("n")

plt.suptitle("bc bcd qmd", fontsize=28)
for ax in axes:
    ax.grid(True)

# ==== カラーパッチと凡例 ====
import matplotlib.patches as mpatches
patches = [mpatches.Patch(color=colors[i], label=str(labels[i])) for i in range(num_files)]
fig.legend(
    handles=patches,
    title="Main mesh size",
    loc='lower center',
    ncol=1,  # 一段表示
    frameon=False,
    fontsize=20,
    title_fontsize=22
)

# ==== 凡例とタイトルに余白を取る ====
plt.tight_layout(rect=[0, 0.14, 1, 0.93])  # 下にスペース追加

# ==== 保存 ====
plt.savefig(f"./figure_qmd/{out_title}.png", dpi=300)
print(f"画像を保存しました")

plt.show()

