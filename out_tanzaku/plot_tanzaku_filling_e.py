import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.cm as cm
import matplotlib.patches as mpatches
import numpy as np
import os

def create_filename_base(prefix, lambda_val, j_val, mesh_x, mesh_y, div, threshold_berry, dir = "data_qmd"):
    """
    設定値からファイル名のベース部分を生成する関数
    """
    # floatを文字列に変換し、'.'を'p'に置換
    lambda_str = str(lambda_val).replace('.', 'p')
    j_str = str(j_val).replace('.', 'p')

    # f-stringを使って最終的な文字列を組み立てる
    filename = (
        f"./{dir}/data_{prefix}_lambda{lambda_str}_j{j_str}_mesh_x{mesh_x}_mesh_y{mesh_y}_div{div}_thresh10em{threshold_berry}.dat"
    )
    
    return filename

# ==== ファイル名リスト ====
file_list = [
    create_filename_base("UuudddTmd", 0.3, 0.25, 1000, 1000, 307, 12,"compare_6_spinmodel"),
    create_filename_base("TwinTmd", 0.3, 0.25, 1000, 1000, 307, 12,"compare_6_spinmodel"),
    create_filename_base("One2Tmd", 0.3, 0.25, 1000, 1000, 307, 12,"compare_6_spinmodel"),
    create_filename_base("Tri1Tmd", 0.3, 0.25, 1000, 1000, 307, 12,"compare_6_spinmodel"),
    create_filename_base("FmTmd", 0.3, 0.25, 1000, 1000, 307, 12,"compare_6_spinmodel"),
    create_filename_base("SatoTmd", 0.3, 0.25, 1000, 1000, 307, 12,"compare_6_spinmodel"),
]

labels = ["uuuddd", "Twin", "One2", "Tri1", "Fm", "Sato"]

row_num = 3

out_title = "stable"


# ==== 色設定 ====
num_files = len(file_list)
colors = cm.rainbow(np.linspace(0, 1, num_files))

# ==== matplotlib全体のフォントサイズ設定 ====
plt.rcParams['font.size'] = 24

# ==== プロット準備 ====
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(14, 16), sharex=True)  # 横幅を広げた

# ==== 全ファイルのy軸範囲をまとめて取得 ====
ymins = np.full(5, np.inf)
ymaxs = np.full(5, -np.inf)

for i, filename in enumerate(file_list):
    file_path = f"{filename}"
    df = pd.read_csv(file_path, comment='#', header=None,
                     names=["n","energy", "bc_sum", "bcd_x_sum", "bcd_y_sum", "qmd_x_sum", "qmd_y_sum"])
    
    df_fm = pd.read_csv(create_filename_base("FmTmd", 0.3, 0.25, 1000, 1000, 307, 12,"compare_6_spinmodel"), comment='#', header=None,
                     names=["n","energy", "bc_sum", "bcd_x_sum", "bcd_y_sum", "qmd_x_sum", "qmd_y_sum"])
    
    color = colors[i]
    ax.plot(df["n"], df["energy"] - df_fm["energy"], color=color)

# ==== ラベルやタイトル ====
ax.set_ylabel("energy")
ax.set_xlabel("n")

plt.suptitle("bc bcd qmd", fontsize=28)
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
plt.savefig(f"./figure_qmd/{out_title}_e.png", dpi=300)
print(f"画像を保存しました")

plt.show()

