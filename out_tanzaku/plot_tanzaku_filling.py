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

def create_filename_base_2(prefix, lambda_val, j_val, mesh_x, mesh_y, div, threshold_berry, main_mesh, dir = "data_qmd"):
    """
    設定値からファイル名のベース部分を生成する関数
    """
    # floatを文字列に変換し、'.'を'p'に置換
    lambda_str = str(lambda_val).replace('.', 'p')
    j_str = str(j_val).replace('.', 'p')

    # f-stringを使って最終的な文字列を組み立てる
    filename = (
        f"./{dir}/data_{prefix}_lambda{lambda_str}_j{j_str}_mesh_x{mesh_x}_mesh_y{mesh_y}_div{div}_thresh10em{threshold_berry}_main_mesh{main_mesh}.dat"
    )
    
    return filename

# # ==== ファイル名リスト ====

# lam = 0.3
# j = 0.25
# file_list = [
#     create_filename_base("UuudddTmd", lam, j, 2000, 2000, 300, 12),
#     create_filename_base_2("UuudddTmd", lam, j, 400, 400, 301, 12,5),
#     create_filename_base_2("UuudddTmd", lam, j, 400, 400, 301, 12,10),
#     create_filename_base_2("UuudddTmd", lam, j, 400, 400, 301, 12,11)
# ]

# labels = [ "mesh 1600","mesh 400 x 5","mesh 400 x 10","mesh 400 x 11"]

# row_num = 5

# out_title = f"UuudddTmd_mesh_compare_2"


# lam = 0.3
# j = 0.10
# mesh = 400
# file_list = [
#     create_filename_base_2("UuudddTmd", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
#     create_filename_base_2("TwinTmd", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
#     create_filename_base_2("One2Tmd", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
#     create_filename_base_2("Tri1Tmd", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
#     create_filename_base_2("Tri2Tmd", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
#     create_filename_base_2("FmTmd", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
#     create_filename_base_2("SatoTmd", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
#     create_filename_base_2("stable", lam, j, mesh, mesh, 307, 12, 8,"compare_6_spinmodel"),
# ]

# labels = ["uuuddd", "Twin", "One2", "Tri1", "Tri2", "Fm", "Sato", "stable"]

# row_num = 5

# out_title = f"stable_lambda{lam}_j{j}_rev"


lam = 0.3
j = 0.1
mesh = 1500
file_list = [
    create_filename_base("UuudddTmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("TwinTmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("One1Tmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("One2Tmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("Tri1Tmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("Tri2Tmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("FmTmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("SatoTmd", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
    create_filename_base("stable", lam, j, mesh, mesh, 307, 12,"compare_6_spinmodel"),
]

labels = ["uuuddd", "uudddd", "uddddd","dudddd", "ududdd", "dududd", "dddddd", "ududud", "stable"]

row_num = 5

out_title = f"stable_lambda{lam}_j{j}_row_num{row_num}"

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
    
    if i == len(file_list) -1:
        line_width = 3
    else:
        line_width = 1

    color = colors[i]
    axes[0].plot(df["n"], df["bc_sum"], color=color, linewidth=line_width)
    axes[1].plot(df["n"], df["bcd_x_sum"], color=color, linewidth=line_width)
    axes[2].plot(df["n"], df["bcd_y_sum"], color=color, linewidth=line_width)

    if row_num == 5 :
        axes[3].plot(df["n"], df["qmd_x_sum"], color=color, linewidth=line_width)
        axes[4].plot(df["n"], df["qmd_y_sum"], color=color, linewidth=line_width)

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

    ax.set_xlim(0.0,1.0)

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
    ncol=3,  # 一段表示
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

